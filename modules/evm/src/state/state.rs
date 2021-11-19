use abcf::bs3::{DoubleKeyMapStore, MapStore, ValueStore};
use ethereum::Log;
use evm::{
    backend::{Backend, Basic},
    executor::stack::{StackState, StackSubstateMetadata},
    Transfer,
};
use primitive_types::{H160, H256, U256};

use crate::EVM_CHAIN_ID;

use super::{basic::AccountBasic, chain::ChainInfo, substate::Substate};

pub struct State<'module, 'config> {
    pub chain_info: &'module dyn ValueStore<ChainInfo>,
    pub block_hashs: &'module dyn MapStore<U256, H256>,
    pub account_basic: &'module mut dyn MapStore<H160, AccountBasic>,
    pub account_storage: &'module mut dyn DoubleKeyMapStore<H160, H256, H256>,

    pub substate: Substate<'config>,
}

impl<'module, 'config> State<'module, 'config> {
    fn get_chain_info(&self) -> ChainInfo {
        let v = self
            .chain_info
            .get()
            .expect("Read EVM chain info from storage failed !");
        let v1 = v.expect("EVM ChainInfo must init when node init.");
        v1.clone()
    }
}

impl<'module, 'config> Backend for State<'module, 'config> {
    fn gas_price(&self) -> U256 {
        let v = self.get_chain_info();
        v.gas_price
    }

    fn origin(&self) -> H160 {
        let v = self.get_chain_info();
        v.origin
    }

    fn block_hash(&self, number: U256) -> H256 {
        let result = self
            .block_hashs
            .get(&number)
            .expect("Read EVM block hash from storge failed!");
        let v = result.expect("EVM block hash must set in begin block.");
        v.clone()
    }

    fn block_number(&self) -> U256 {
        let v = self.get_chain_info();
        v.block_number
    }

    fn block_coinbase(&self) -> H160 {
        let v = self.get_chain_info();
        v.block_coinbase
    }

    fn block_timestamp(&self) -> U256 {
        let v = self.get_chain_info();
        v.block_timestamp
    }

    fn block_gas_limit(&self) -> U256 {
        let v = self.get_chain_info();
        v.block_gas_limit
    }

    fn block_difficulty(&self) -> U256 {
        let v = self.get_chain_info();
        v.block_difficulty
    }

    fn block_base_fee_per_gas(&self) -> U256 {
        let v = self.get_chain_info();
        v.block_base_fee_per_gas
    }

    fn chain_id(&self) -> U256 {
        EVM_CHAIN_ID
    }

    fn exists(&self, address: H160) -> bool {
        let result = self
            .account_basic
            .get(&address)
            .expect("Read EVM account basic from storge failed!");
        result.is_some()
    }

    fn code(&self, address: H160) -> Vec<u8> {
        let result = self
            .account_basic
            .get(&address)
            .expect("Read EVM account basic from storge failed!");
        if let Some(v) = result {
            v.code.clone()
        } else {
            Vec::new()
        }
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        let result = self
            .account_storage
            .get(&address, &index)
            .expect("Read EVM account storage from storge failed!");

        if let Some(v) = result {
            v.clone()
        } else {
            Default::default()
        }
    }

    fn basic(&self, address: H160) -> Basic {
        let result = self
            .account_basic
            .get(&address)
            .expect("Read EVM account basic from storge failed!");

        let nonce = if let Some(v) = result {
            v.nonce.clone()
        } else {
            Default::default()
        };

        Basic {
            balance: Default::default(),
            nonce,
        }
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        let result = self
            .account_storage
            .get(&address, &index)
            .expect("Read EVM account storage from storge failed!");

        result.map(|v| v.clone())
    }
}

impl<'module, 'config> StackState<'config> for State<'module, 'config> {
    fn metadata(&self) -> &StackSubstateMetadata<'config> {
        &self.substate.metadata
    }

    fn metadata_mut(&mut self) -> &mut StackSubstateMetadata<'config> {
        &mut self.substate.metadata
    }

    fn enter(&mut self, gas_limit: u64, is_static: bool) {
        self.substate.start(gas_limit, is_static)
    }

    fn exit_commit(&mut self) -> Result<(), evm::ExitError> {
        Ok(())
    }

    fn exit_revert(&mut self) -> Result<(), evm::ExitError> {
        Ok(())
    }

    fn exit_discard(&mut self) -> Result<(), evm::ExitError> {
        Ok(())
    }

    fn is_cold(&self, _address: H160) -> bool {
        // No impl.
        false
    }

    fn is_empty(&self, address: H160) -> bool {
        let result = self
            .account_basic
            .get(&address)
            .expect("Read EVM account basic from storge failed!");
        if let Some(v) = result {
            v.nonce == U256::zero() && v.code.len() == 0
        } else {
            true
        }
    }

    fn deleted(&self, _address: H160) -> bool {
        // TODO: impl in substate
        false
    }

    fn inc_nonce(&mut self, address: H160) {
        let result = self
            .account_basic
            .get_mut(&address)
            .expect("Read EVM account basic from storge failed!");

        if let Some(v) = result {
            v.nonce.saturating_add(1.into());
        } else {
            let basic = AccountBasic {
                nonce: 1.into(),
                code: Vec::new(),
            };
            self.account_basic
                .insert(address, basic)
                .expect("Write EVM account basic to storage failed !");
        }
    }

    fn set_code(&mut self, address: H160, code: Vec<u8>) {
        let result = self
            .account_basic
            .get_mut(&address)
            .expect("Read EVM account basic from storage failed!");

        if let Some(v) = result {
            v.code = code;
        } else {
            let basic = AccountBasic {
                nonce: 0.into(),
                code,
            };
            self.account_basic
                .insert(address, basic)
                .expect("Write EVM account basic to storage failed !");
        }
    }

    fn set_storage(&mut self, address: H160, key: H256, value: H256) {
        let result = self
            .account_storage
            .get_mut(&address, &key)
            .expect("Read EVM account storage from storage failed!");
        if let Some(v) = result {
            *v = value;
        } else {
            self.account_storage
                .insert(address, key, value)
                .expect("Write EVM account basic to storage failed !");
        }
    }

    fn set_deleted(&mut self, _address: H160) {
        // TODO: impl in substate.
    }

    fn reset_storage(&mut self, _address: H160) {
        // TODO: impl need bs3 support.
    }

    fn log(&mut self, address: H160, topics: Vec<H256>, data: Vec<u8>) {
        self.substate.log(Log {
            address,
            topics,
            data,
        })
    }

    fn transfer(&mut self, _transfer: Transfer) -> Result<(), evm::ExitError> {
        // TODO: deal transfer.
        Ok(())
    }

    fn touch(&mut self, address: H160) {
        let result = self
            .account_basic
            .get(&address)
            .expect("Read EVM account basic from storage failed!");

        if let None = result {
            let basic = AccountBasic {
                nonce: 0.into(),
                code: Vec::new(),
            };
            self.account_basic
                .insert(address, basic)
                .expect("Write EVM account basic to storage failed !");
        }
    }

    fn reset_balance(&mut self, _address: H160) {
        // No impl.
        // Beacuse our account is a ERC20.
    }

    fn is_storage_cold(&self, _address: H160, _key: H256) -> bool {
        // no impl
        false
    }
}
