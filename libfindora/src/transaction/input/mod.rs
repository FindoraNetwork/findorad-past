use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub txid: Vec<u8>,
    pub n: u32,
    pub operation: Operation,
}
