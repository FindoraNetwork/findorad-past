use serde::{Deserialize, Serialize};
use zei::xfr::sig::XfrPublicKey;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetDelegationInfoReq {
    pub owners: Vec<XfrPublicKey>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GetDelegationInfoResp {
    pub pub_key: String,
    pub infos: Vec<GDIEntry>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct GDIEntry {
    pub validator: String,
    pub amount: u64,
    pub power: u64,
}
