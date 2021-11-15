use crate::account;

#[derive(Debug, Clone)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
    Undelegate,
    ClaimReward,
    TransferAccount(account::InputOperation),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: Vec<u8>,
    pub n: u32,
    pub operation: Operation,
}
