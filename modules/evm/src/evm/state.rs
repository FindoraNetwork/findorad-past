use abcf::bs3::MapStore;
use evm::{
    backend::{Backend, Basic},
    executor::stack::StackSubstateMetadata,
};
use primitive_types::{H160, H256, U256};

use super::{account::Account, vicinity::Vicinity};

pub struct SubstackState<'config, A, S, D, L> {
    pub metadata: StackSubstateMetadata<'config>,
    pub accounts: A,
    pub storages: S,
    pub deletes: D,
    pub logs: L,
}

pub struct StackState<'config, A, S, D, L> {
    pub vicinity: Vicinity,
    pub substacks: Vec<SubstackState<'config, A, S, D, L>>,
}

impl<'config, A, S, D, L> StackState<'config, A, S, D, L> {
    fn latest_substate(&self) -> &SubstackState<'config, A, S, D, L> {
        let index = self.substacks.len() - 1;
        &self.substacks[index]
    }
}

impl<'config, A: MapStore<H160, Account>, S, D, L> Backend for StackState<'config, A, S, D, L> {
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }
    fn origin(&self) -> H160 {
        self.vicinity.origin
    }
    fn block_hash(&self, number: U256) -> H256 {
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
                log::error!("read account error: {}", 1);
                false
            }
        }
    }

    fn basic(&self, address: H160) -> Basic {
        self.substate
            .known_basic(address)
            .unwrap_or_else(|| self.vicinity.basic(address))
    }

    fn code(&self, address: H160) -> Vec<u8> {
        self.substate
            .known_code(address)
            .unwrap_or_else(|| self.vicinity.code(address))
    }

    fn storage(&self, address: H160, key: H256) -> H256 {
        self.substate
            .known_storage(address, key)
            .unwrap_or_else(|| self.vicinity.storage(address, key))
    }

    fn original_storage(&self, address: H160, key: H256) -> Option<H256> {
        if let Some(value) = self.substate.known_original_storage(address, key) {
            return Some(value);
        }

        self.vicinity.original_storage(address, key)
    }
}
