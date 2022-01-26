use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataResponse {
    pub chain_id: u64,
    pub gas_price: u64,
}
