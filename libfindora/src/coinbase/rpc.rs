use serde::{Deserialize, Serialize};
use zei::xfr::{sig::XfrPublicKey, structs::AssetType};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetOwnerReq {
    pub asset_type: AssetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetOwnerResp {
    pub owner: Option<XfrPublicKey>,
}
