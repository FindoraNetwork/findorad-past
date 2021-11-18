use primitive_types::{H160, U256};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChainInfo {
    pub gas_price: U256,
    pub origin: H160,
    pub block_number: U256,
    pub block_coinbase: H160,
    pub block_timestamp: U256,
    pub block_difficulty: U256,
    pub block_gas_limit: U256,
    pub block_base_fee_per_gas: U256,
}
