use std::collections::{BTreeMap, BTreeSet};

use abcf::bs3::{Forkable, MapStore};
use ethereum::Log;
use evm::{
    backend::{Backend, Basic},
    executor::stack::{StackState, StackSubstateMetadata},
    ExitError,
};
use libfindora::{
    asset::{XfrAmount, FRA},
    utxo::{Output, OutputId},
    Address,
};
use primitive_types::{H160, H256, H512, U256};

use crate::utils;

use super::{account::Account, vicinity::Vicinity};

pub struct SubstackState<'config, A, S, OO, OS> {
    pub metadata: StackSubstateMetadata<'config>,
    pub accounts: A,
    pub storages: S,
    pub owned_outputs: OO,
    pub outputs_set: OS,
    pub logs: Vec<Log>,
    pub deletes: BTreeSet<H160>,
}

pub struct State<'config, A, S, OO, OS> {
    pub vicinity: Vicinity,
    pub substates: Vec<SubstackState<'config, A, S, OO, OS>>,
    pub txid: H512,
    pub n: u32,
}

impl<
        'config,
        A: MapStore<H160, Account>,
        S: MapStore<H160, BTreeMap<H256, H256>>,
        OO: MapStore<Address, Vec<OutputId>>,
        OS: MapStore<OutputId, Output>,
    > State<'config, A, S, OO, OS>
{
    fn latest_substate(&self) -> &SubstackState<'config, A, S, OO, OS> {
        let index = self.substates.len() - 1;
        &self.substates[index]
    }

    fn latest_substate_mut(&mut self) -> &mut SubstackState<'config, A, S, OO, OS> {
        let index = self.substates.len() - 1;
        &mut self.substates[index]
    }
    fn basic_resulted(&self, address: H160) -> crate::Result<Basic> {
        let ua = Address::from(address);
        let balance = if let Some(v) = self.latest_substate().owned_outputs.get(&ua)? {
            let mut balance = 0;

            for output_id in v.as_ref() {
                if let Some(output) = self.latest_substate().outputs_set.get(output_id)? {
                    if let XfrAmount::NonConfidential(e) = &output.amount {
                        balance += e;
                    }
                }
            }

            balance
        } else {
            0
        };

        let nonce = match self.latest_substate().accounts.get(&address)? {
            Some(e) => e.nonce,
            None => 0,
        };

        Ok(Basic {
            balance: U256::from(balance),
            nonce: U256::from(nonce),
        })
    }
}

impl<
        'config,
        A: MapStore<H160, Account>,
        S: MapStore<H160, BTreeMap<H256, H256>>,
        OO: MapStore<Address, Vec<OutputId>>,
        OS: MapStore<OutputId, Output>,
    > Backend for State<'config, A, S, OO, OS>
{
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }
    fn origin(&self) -> H160 {
        self.vicinity.origin
    }
    fn block_hash(&self, _number: U256) -> H256 {
        self.vicinity.block_hash
    }
    fn block_number(&self) -> U256 {
        self.vicinity.block_number
    }
    fn block_coinbase(&self) -> H160 {
        self.vicinity.block_coinbase
    }
    fn block_timestamp(&self) -> U256 {
        self.vicinity.block_timestamp
    }
    fn block_difficulty(&self) -> U256 {
        self.vicinity.block_difficulty
    }
    fn block_gas_limit(&self) -> U256 {
        self.vicinity.block_gas_limit
    }
    fn block_base_fee_per_gas(&self) -> U256 {
        self.vicinity.block_base_fee_per_gas
    }

    fn chain_id(&self) -> U256 {
        self.vicinity.chain_id
    }

    fn exists(&self, address: H160) -> bool {
        match self.latest_substate().accounts.get(&address) {
            Ok(e) => e.is_some(),
            Err(e) => {
                log::error!("read account error: {:?}", e);
                false
            }
        }
    }

    fn basic(&self, address: H160) -> Basic {
        match self.basic_resulted(address) {
            Ok(e) => e,
            Err(e) => {
                log::error!("read basic error: {:?}", e);
                Basic::default()
            }
        }
    }

    fn code(&self, address: H160) -> Vec<u8> {
        match self.latest_substate().accounts.get(&address) {
            Ok(Some(e)) => e.code.clone(),
            Ok(None) => Vec::new(),
            Err(e) => {
                log::error!("read code error: {:?}", e);
                Vec::new()
            }
        }
    }

    fn storage(&self, address: H160, key: H256) -> H256 {
        match self.original_storage(address, key) {
            Some(e) => e,
            None => H256::default(),
        }
    }

    fn original_storage(&self, address: H160, key: H256) -> Option<H256> {
        match self.latest_substate().storages.get(&address) {
            Ok(Some(e)) => e.get(&key).copied(),
            Ok(None) => None,
            Err(e) => {
                log::error!("read code error: {:?}", e);
                None
            }
        }
    }
}

impl<
        'config,
        A: MapStore<H160, Account> + Clone + Forkable,
        S: MapStore<H160, BTreeMap<H256, H256>> + Clone + Forkable,
        OO: MapStore<Address, Vec<OutputId>> + Clone + Forkable,
        OS: MapStore<OutputId, Output> + Clone + Forkable,
    > State<'config, A, S, OO, OS>
{
    fn _enter(&mut self, gas_limit: u64, is_static: bool) {
        let latest_substate = self.latest_substate_mut();

        let metadata = latest_substate.metadata.spit_child(gas_limit, is_static);

        let accounts = latest_substate.accounts.clone();
        let logs = std::mem::take(&mut latest_substate.logs);
        let deletes = std::mem::take(&mut latest_substate.deletes);
        let outputs_set = latest_substate.outputs_set.clone();
        let owned_outputs = latest_substate.owned_outputs.clone();
        let storages = latest_substate.storages.clone();

        let substate = SubstackState {
            accounts,
            logs,
            deletes,
            metadata,
            outputs_set,
            owned_outputs,
            storages,
        };
        self.substates.push(substate);
    }

    fn _exit_commit(&mut self) -> Result<(), ExitError> {
        if let Some(mut pop_substate) = self.substates.pop() {
            let latest_substate = self.latest_substate_mut();

            latest_substate
                .metadata
                .swallow_commit(pop_substate.metadata)?;
            latest_substate.logs.append(&mut pop_substate.logs);
            latest_substate.deletes.append(&mut pop_substate.deletes);

            latest_substate
                .accounts
                .merge(pop_substate.accounts.cache());
            latest_substate
                .outputs_set
                .merge(pop_substate.outputs_set.cache());
            latest_substate
                .owned_outputs
                .merge(pop_substate.owned_outputs.cache());
            latest_substate
                .storages
                .merge(pop_substate.storages.cache());
        } else {
            return Err(ExitError::Other("Cannot commit on root substate".into()));
        }
        Ok(())
    }

    fn _exit_revert(&mut self) -> Result<(), ExitError> {
        if let Some(pop_substate) = self.substates.pop() {
            let latest_substate = self.latest_substate_mut();

            latest_substate
                .metadata
                .swallow_revert(pop_substate.metadata)?;
        } else {
            return Err(ExitError::Other("Cannot commit on root substate".into()));
        }

        Ok(())
    }

    fn _exit_discard(&mut self) -> Result<(), ExitError> {
        if let Some(pop_substate) = self.substates.pop() {
            let latest_substate = self.latest_substate_mut();

            latest_substate
                .metadata
                .swallow_discard(pop_substate.metadata)?;
        } else {
            return Err(ExitError::Other("Cannot commit on root substate".into()));
        }

        Ok(())
    }

    fn _is_empty(&self, address: H160) -> crate::Result<bool> {
        let r0 = if let Some(v) = self
            .latest_substate()
            .owned_outputs
            .get(&Address::from(address))?
        {
            v.len() == 0
        } else {
            true
        };

        let r1 = if let Some(account) = self.latest_substate().accounts.get(&address)? {
            account.code.is_empty() && account.nonce == 0
        } else {
            true
        };

        Ok(r0 && r1)
    }

    fn _deleted(&self, address: H160) -> bool {
        for state in self.substates.iter().rev() {
            if state.deletes.contains(&address) {
                return true;
            }
        }
        false
    }

    fn _is_cold(&self, address: H160) -> bool {
        for substate in self.substates.iter().rev() {
            let local_is_accessed = substate
                .metadata
                .accessed()
                .as_ref()
                .map(|a| a.accessed_addresses.contains(&address))
                .unwrap_or(false);

            if local_is_accessed {
                return false;
            }
        }

        true
    }

    fn _is_storage_cold(&self, address: H160, key: H256) -> bool {
        for substate in self.substates.iter().rev() {
            let local_is_accessed = substate
                .metadata
                .accessed()
                .as_ref()
                .map(|a| a.accessed_storage.contains(&(address, key)))
                .unwrap_or(false);

            if local_is_accessed {
                return false;
            }
        }

        true
    }

    fn _inc_nonce(&mut self, address: H160) -> crate::Result<()> {
        let accounts = &mut self.latest_substate_mut().accounts;

        if let Some(e) = accounts.get_mut(&address)? {
            e.nonce += 1;
        } else {
            accounts.insert(
                address,
                Account {
                    code: Vec::new(),
                    nonce: 1,
                },
            )?;
        }

        Ok(())
    }

    fn _set_storage(&mut self, address: H160, key: H256, value: H256) -> crate::Result<()> {
        if let Some(m) = self.latest_substate_mut().storages.get_mut(&address)? {
            m.insert(key, value);
        } else {
            let mut m = BTreeMap::new();
            m.insert(key, value);
            self.latest_substate_mut().storages.insert(address, m)?;
        }
        Ok(())
    }

    fn _reset_storage(&mut self, address: H160) -> crate::Result<()> {
        if let Some(m) = self.latest_substate_mut().storages.get_mut(&address)? {
            std::mem::take(m);
        }
        Ok(())
    }

    fn _log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {
        self.latest_substate_mut().logs.push(Log {
            address,
            topics,
            data,
        });
    }

    fn _set_deleted(&mut self, address: H160) {
        self.latest_substate_mut().deletes.insert(address);
    }

    fn _set_code(&mut self, address: H160, code: Vec<u8>) -> crate::Result<()> {
        if let Some(v) = self.latest_substate_mut().accounts.get_mut(&address)? {
            v.code = code;
        } else {
            self.latest_substate_mut()
                .accounts
                .insert(address, Account { nonce: 0, code })?;
        }
        Ok(())
    }

    fn _transfer(&mut self, transfer: evm::Transfer) -> crate::Result<()> {
        let oid = OutputId {
            txid: self.txid,
            n: self.n,
        };

        let latest_substate = self.latest_substate_mut();

        let from = Address::from(transfer.source);
        let to = Address::from(transfer.target);
        let amount = transfer.value.as_u64();
        let asset = FRA.bare_asset_type;

        utils::transfer(
            from,
            to,
            amount,
            asset,
            oid,
            &mut latest_substate.outputs_set,
            &mut latest_substate.owned_outputs,
        )?;

        self.n += 1;

        Ok(())
    }

    fn _reset_balance(&mut self, address: H160) -> crate::Result<()> {
        let output_ids = if let Some(v) = self
            .latest_substate_mut()
            .owned_outputs
            .get_mut(&Address::from(address))?
        {
            std::mem::take(v)
        } else {
            Vec::new()
        };

        for output_id in output_ids {
            self.latest_substate_mut().outputs_set.remove(&output_id)?;
        }

        Ok(())
    }

    fn _touch(&mut self, address: H160) -> crate::Result<()> {
        self.latest_substate_mut().accounts.insert(
            address,
            Account {
                code: Vec::new(),
                nonce: 0,
            },
        )?;
        Ok(())
    }
}

impl<
        'config,
        A: MapStore<H160, Account> + Clone + Forkable,
        S: MapStore<H160, BTreeMap<H256, H256>> + Clone + Forkable,
        OO: MapStore<Address, Vec<OutputId>> + Clone + Forkable,
        OS: MapStore<OutputId, Output> + Clone + Forkable,
    > StackState<'config> for State<'config, A, S, OO, OS>
{
    fn metadata(&self) -> &StackSubstateMetadata<'config> {
        &self.latest_substate().metadata
    }

    fn metadata_mut(&mut self) -> &mut StackSubstateMetadata<'config> {
        &mut self.latest_substate_mut().metadata
    }

    fn enter(&mut self, gas_limit: u64, is_static: bool) {
        self._enter(gas_limit, is_static)
    }

    fn exit_commit(&mut self) -> Result<(), ExitError> {
        self._exit_commit()
    }

    fn exit_revert(&mut self) -> Result<(), ExitError> {
        self._exit_revert()
    }

    fn exit_discard(&mut self) -> Result<(), ExitError> {
        self._exit_discard()
    }

    fn is_empty(&self, address: H160) -> bool {
        match self._is_empty(address) {
            Ok(e) => e,
            Err(e) => {
                log::error!("read account error: {:?}", e);
                true
            }
        }
    }

    fn deleted(&self, address: H160) -> bool {
        self._deleted(address)
    }

    fn is_cold(&self, address: H160) -> bool {
        self._is_cold(address)
    }

    fn is_storage_cold(&self, address: H160, key: H256) -> bool {
        self._is_storage_cold(address, key)
    }

    fn inc_nonce(&mut self, address: H160) {
        if let Err(e) = self._inc_nonce(address) {
            log::error!("increase nonce error: {:?}", e);
        }
    }

    fn set_storage(&mut self, address: H160, key: H256, value: H256) {
        if let Err(e) = self._set_storage(address, key, value) {
            log::error!("set address error: {:?}", e);
        }
    }

    fn reset_storage(&mut self, address: H160) {
        if let Err(e) = self._reset_storage(address) {
            log::error!("reset storage error: {:?}", e);
        }
    }

    fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {
        self._log(address, topics, data)
    }

    fn set_deleted(&mut self, address: H160) {
        self._set_deleted(address)
    }

    fn set_code(&mut self, address: H160, code: Vec<u8>) {
        if let Err(e) = self._set_code(address, code) {
            log::error!("set code error: {:?}", e);
        }
    }

    fn transfer(&mut self, transfer: evm::Transfer) -> Result<(), ExitError> {
        self._transfer(transfer)
            .map_err(|e| ExitError::Other(format!("{:?}", e).into()))
    }

    fn reset_balance(&mut self, address: H160) {
        if let Err(e) = self._reset_balance(address) {
            log::error!("reset account error: {:?}", e);
        }
    }

    fn touch(&mut self, address: H160) {
        if let Err(e) = self._touch(address) {
            log::error!("create account error: {:?}", e);
        }
    }
}
