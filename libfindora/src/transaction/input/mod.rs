use serde::{Deserialize, Serialize};
use zei::xfr::sig::XfrSignature;

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
    pub signature: Option<XfrSignature>,
}
