use serde::{Deserialize, Serialize};

use super::{transaction::Output, Address, OutputId};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoReq {
    pub owners: Vec<Address>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OwnedOutput {
    pub output_id: OutputId,
    pub output: Output,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoResp {
    pub outputs: Vec<(usize, OwnedOutput)>,
}
