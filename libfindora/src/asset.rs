use zei::xfr::structs::ASSET_TYPE_LENGTH;
pub use zei::xfr::structs::{AssetType, XfrAmount, XfrAssetType};

pub struct FraAsset {
    pub bare_asset_type: AssetType,
    pub asset_type: XfrAssetType,
    pub decimals: u8,
    pub units: Amount,
}

const FRA_BARE_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
const FRA_DECIMALS: u8 = 6;

pub const FRA: FraAsset = FraAsset {
    bare_asset_type: FRA_BARE_ASSET_TYPE,
    asset_type: XfrAssetType::NonConfidential(FRA_BARE_ASSET_TYPE),
    decimals: FRA_DECIMALS,
    units: 10_u64.pow(FRA_DECIMALS as u32),
};

pub type Amount = u64;

use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssetMeta {
    pub maximum: Option<U256>,
    pub transferable: bool,
}
