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
