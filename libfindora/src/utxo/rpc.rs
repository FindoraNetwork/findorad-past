use serde::{Deserialize, Serialize};
use zei::xfr::sig::XfrPublicKey;

use super::{transaction::Output, OutputId};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoReq {
    pub owners: Vec<XfrPublicKey>,
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

