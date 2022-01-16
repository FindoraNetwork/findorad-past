use abcf::bs3::{DoubleKeyMapStore, MapStore};
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
}

pub struct State<'config, A, S, OO, OS> {
    pub vicinity: Vicinity,
    pub substacks: Vec<SubstackState<'config, A, S, OO, OS>>,
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
        let r0 = false;

        let r1 = match self.latest_substate().accounts.get(&address) {
            Ok(Some(account)) => account.code.len() == 0 && account.nonce == 0,
            Ok(None) => true,
            Err(e) => {
                log::error!("read account error: {:?}", e);
                true
            }
        };

        r0 && r1
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

    fn inc_nonce(&mut self, address: H160) {
        let accounts = &mut self.latest_substate_mut().accounts;

        match accounts.get_mut(&address) {
            Ok(Some(e)) => e.nonce += 1,
            Ok(None) => {
                accounts.insert(
                    address,
                    Account {
                        code: Vec::new(),
                        nonce: 1,
                        reset: false,
                    },
                );
            }
            Err(e) => {
                log::error!("read account error: {:?}", e);
            }
        }
    }

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
