use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
    Undelegate,
    ClaimReward,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub txid: Vec<u8>,
    pub n: u32,
    pub operation: Operation,
}
