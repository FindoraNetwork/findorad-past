use serde::{Deserialize, Serialize};
use zei::xfr::{sig::XfrPublicKey, structs::BlindAssetRecord};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoReq {
    pub owner: XfrPublicKey,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedUtxoResp {
    pub outputs: Vec<BlindAssetRecord>,
}
