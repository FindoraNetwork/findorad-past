use serde::{Deserialize, Serialize};
use zei::xfr::{structs::BlindAssetRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoReq {
    pub owner: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OwnedOutput {
    pub txid: Vec<u8>,
    pub n: u32,
    pub core: BlindAssetRecord,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoResp {
    pub outputs: Vec<OwnedOutput>,
}
