use serde::{Deserialize, Serialize};
use zei::xfr::sig::XfrPublicKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedStakingReq {
    pub owners: Vec<XfrPublicKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedStakingResp {
    pub pub_key: String,
    pub infos: Vec<GOSInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GOSInfo {
    pub validator: String,
    pub amount: u64,
    pub power: u64,
}