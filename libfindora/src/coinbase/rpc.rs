use serde::{Deserialize, Serialize};
use zei::xfr::structs::AssetType;

use crate::utxo::Address;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetOwnerReq {
    pub asset_type: AssetType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetOwnerResp {
    pub owner: Option<Address>,
}
