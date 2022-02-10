use libfindora::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataRequest {}

#[derive(Serialize, Deserialize, Debug)]
pub struct MetadataResponse {
    pub chain_id: u64,
    pub gas_price: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CallRequest {
    pub from: Address,
    pub to: Address,
    pub nonce: u64,
    pub amount: u64,
    pub data: Vec<u8>,
    pub gas_limit: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct CallResponse {
    pub gas: u64,
    // TODO: Add hex support
    pub data: Vec<u8>,
}
