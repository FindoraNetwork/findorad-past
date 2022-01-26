use primitive_types::{H160, H256, U256};

pub struct Vicinity {
    pub gas_price: U256,
    pub origin: H160,
    pub chain_id: U256,
    pub block_hash: H256,
    pub block_number: U256,
    pub block_coinbase: H160,
    pub block_timestamp: U256,
    pub block_difficulty: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
}

impl Vicinity {
    pub fn mainnet() -> Self {
        Vicinity {
            gas_price: U256::from(21000),
            origin: Default::default(),
            chain_id: U256::from(0x868),
            block_hash: Default::default(),
            block_number: Default::default(),
            block_coinbase: Default::default(),
            block_timestamp: Default::default(),
            block_difficulty: Default::default(),
            block_gas_limit: Default::default(),
            block_base_fee_per_gas: Default::default(),
        }
    }

    pub fn testnet() -> Self {
        Vicinity {
            gas_price: U256::from(21000),
            origin: Default::default(),
            chain_id: U256::from(0x869),
            block_hash: Default::default(),
            block_number: Default::default(),
            block_coinbase: Default::default(),
            block_timestamp: Default::default(),
            block_difficulty: Default::default(),
            block_gas_limit: Default::default(),
            block_base_fee_per_gas: Default::default(),
        }
    }
}

