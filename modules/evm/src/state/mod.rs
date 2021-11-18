mod chain;

use abcf::bs3::{MapStore, ValueStore};
use evm::{
    backend::{Backend, Basic},
    executor::stack::StackState,
};
use primitive_types::{H160, H256, U256};

use crate::EVM_CHAIN_ID;

pub struct State<'module> {
    pub chain_info: &'module dyn ValueStore<chain::ChainInfo>,
    pub block_hashs: &'module dyn MapStore<U256, H256>,
    // pub substrate_metadata: StackSubstateMetadata<'config>,
}

impl<'module> State<'module> {
    fn get_chain_info(&self) -> chain::ChainInfo {
        let v = self
            .chain_info
            .get()
            .expect("Read EVM chain info from storage failed !");
        let v1 = v.expect("EVM ChainInfo must init when node init.");
        v1.clone()
    }
}

impl<'module> Backend for State<'module> {
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
        false
    }

    fn code(&self, address: H160) -> Vec<u8> {
        Vec::new()
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        Default::default()
    }

    fn basic(&self, address: H160) -> evm::backend::Basic {
        Basic {
            balance: Default::default(),
            nonce: Default::default(),
        }
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        Default::default()
    }
}

// impl<'config> StackState for State<'config> {
//
// }
//
