use primitive_types::H512;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
    Undelegate,
    ClaimReward,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub txid: H512,
    pub n: u32,
    pub operation: Operation,
}
