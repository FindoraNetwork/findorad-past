use zei::xfr::sig::XfrPublicKey;
use crate::common::{AssetType, AssetTypeCode, DefineAsset, Issuances};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedAssetReq {
    pub owner: Vec<XfrPublicKey>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOwnedAssetResp {
    pub resp: Vec<(String, Vec<DefineAsset>)>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetTypeReq {
    pub asset_type_code: Vec<AssetTypeCode>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetAssetTypeResp {
    pub resp: Vec<Option<AssetType>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenCodeReq {
    pub asset_type_code: AssetTypeCode
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTokenCodeResp {
    pub resp: Option<Issuances>,
}