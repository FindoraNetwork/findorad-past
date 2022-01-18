use abcf::bs3::{DoubleKeyMapStore, MapStore};
use ethereum::Log;
use evm::{
    backend::{Backend, Basic},
    executor::stack::{StackState, StackSubstateMetadata},
    ExitError, Transfer,
};
use libfindora::{
    asset::XfrAmount,
    utxo::{Output, OutputId},
    Address,
};
use primitive_types::{H160, H256, U256};

use super::{account::Account, vicinity::Vicinity};

pub struct SubstackState<'config, A, S, OO, OS> {
    pub metadata: StackSubstateMetadata<'config>,
    pub accounts: A,
    pub storages: S,
    pub owned_outputs: OO,
    pub outputs_set: OS,
    pub logs: Vec<Log>,
}

pub struct State<'config, A, S, OO, OS> {
    pub vicinity: Vicinity,
    pub substacks: Vec<SubstackState<'config, A, S, OO, OS>>,
    pub logs: Vec<Log>,
}

impl<
        'config,
        A: MapStore<H160, Account>,
        S,
        OO: MapStore<Address, Vec<OutputId>>,
        OS: MapStore<OutputId, Output>,
    > State<'config, A, S, OO, OS>
{
    fn latest_substate(&self) -> &SubstackState<'config, A, S, OO, OS> {
        let index = self.substacks.len() - 1;
        &self.substacks[index]
    }

    fn latest_substate_mut(&mut self) -> &mut SubstackState<'config, A, S, OO, OS> {
        let index = self.substacks.len() - 1;
        &mut self.substacks[index]
    }
    fn basic_resulted(&self, address: H160) -> crate::Result<Basic> {
        let ua = Address::from(address);
        let balance = if let Some(v) = self.latest_substate().owned_outputs.get(&ua)? {
            let mut balance = 0;

            for output_id in v.as_ref() {
                if let Some(output) = self.latest_substate().outputs_set.get(&output_id)? {
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
        S: DoubleKeyMapStore<H160, H256, H256>,
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
        // let substate =
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
        match self.latest_substate().storages.get(&address, &key) {
            Ok(Some(e)) => Some(e.clone()),
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
        A: MapStore<H160, Account>,
        S: DoubleKeyMapStore<H160, H256, H256>,
        OO: MapStore<Address, Vec<OutputId>>,
        OS: MapStore<OutputId, Output>,
    > State<'config, A, S, OO, OS>
{
    fn _enter(&mut self, gas_limit: u64, is_static: bool) {
        // enter stack.
    }

    fn _exit_commit(&mut self) -> Result<(), ExitError> {
        // commit stack.
        Ok(())
    }

    fn _exit_revert(&mut self) -> Result<(), ExitError> {
        // revert stack.
        Ok(())
    }

    fn _exit_discard(&mut self) -> Result<(), ExitError> {
        // discard stack.
        Ok(())
    }

    fn _is_empty(&self, address: H160) -> crate::Result<bool> {
        let r0 = if let Some(v) = self.latest_substate().owned_outputs.get(&Address::from(address))? {
            v.len() == 0
        } else {
            true
        };

        let r1 = if let Some(account) = self.latest_substate().accounts.get(&address)? {
            account.code.len() == 0 && account.nonce == 0
        } else {
            true
        };

        Ok(r0 && r1)
    }

    fn _deleted(&self, address: H160) -> bool {
        false
    }

    fn _is_cold(&self, address: H160) -> bool {
        false
    }

    fn _is_storage_cold(&self, address: H160, key: H256) -> bool {
        false
    }

    fn _inc_nonce(&mut self, address: H160) -> crate::Result<()> {
        let accounts = &mut self.latest_substate_mut().accounts;

        if let Some(e) = accounts.get_mut(&address)? {
            e.nonce += 1;
        } else {
            accounts.insert(address, Account {
                code: Vec::new(),
                nonce: 1,
            })?;
        }

        Ok(())
    }

    fn _set_storage(&mut self, address: H160, key: H256, value: H256) -> crate::Result<()> {
        self.latest_substate_mut().storages.insert(address, key, value)?;
        Ok(())
    }

    fn _reset_storage(&mut self, address: H160) {}

    fn _log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {
        self.latest_substate_mut().logs.push(Log {
            address, topics, data,
        });
    }

    fn _set_deleted(&mut self, address: H160) {}

    fn _set_code(&mut self, address: H160, code: Vec<u8>) -> crate::Result<()> {
        if let Some(v) = self.latest_substate_mut().accounts.get_mut(&address)? {
            v.code = code;
        } else {
            self.latest_substate_mut().accounts.insert(address, Account {
                nonce: 0,
                code,
            })?;
        }
        Ok(())
    }

    fn _transfer(&mut self, transfer: Transfer) -> Result<(), ExitError> {
        Ok(())
    }

    fn _reset_balance(&mut self, address: H160) -> crate::Result<()> {
        let output_ids = if let Some(v) = self.latest_substate_mut().owned_outputs.get_mut(&Address::from(address))? {
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
        self.latest_substate_mut().accounts.insert(address, Account {
            code: Vec::new(),
            nonce: 0,
        })?;
        Ok(())
    }
}

impl<
        'config,
        A: MapStore<H160, Account>,
        S: DoubleKeyMapStore<H160, H256, H256>,
        OO: MapStore<Address, Vec<OutputId>>,
        OS: MapStore<OutputId, Output>,
    > StackState<'config> for State<'config, A, S, OO, OS>
{
    fn metadata(&self) -> &StackSubstateMetadata<'config> {
        &self.latest_substate().metadata
    }

    fn metadata_mut(&mut self) -> &mut StackSubstateMetadata<'config> {
        &mut self.latest_substate_mut().metadata
    }

    fn enter(&mut self, gas_limit: u64, is_static: bool) {
        // enter stack.
    }

    fn exit_commit(&mut self) -> Result<(), ExitError> {
        // commit stack.
        Ok(())
    }

    fn exit_revert(&mut self) -> Result<(), ExitError> {
        // revert stack.
        Ok(())
    }

    fn exit_discard(&mut self) -> Result<(), ExitError> {
        // discard stack.
        Ok(())
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
        false
    }

    fn is_cold(&self, address: H160) -> bool {
        false
    }

    fn is_storage_cold(&self, address: H160, key: H256) -> bool {
        false
    }

    fn inc_nonce(&mut self, address: H160) {}

    fn set_storage(&mut self, address: H160, key: H256, value: H256) {}

    fn reset_storage(&mut self, address: H160) {}

    fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {}

    fn set_deleted(&mut self, address: H160) {}

    fn set_code(&mut self, address: H160, code: Vec<u8>) {}

    fn transfer(&mut self, transfer: Transfer) -> Result<(), ExitError> {
        Ok(())
    }

    fn reset_balance(&mut self, address: H160) {}

    fn touch(&mut self, address: H160) {
        // Empty impl.
    }
}
