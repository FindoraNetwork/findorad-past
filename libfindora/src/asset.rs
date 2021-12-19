use zei::xfr::structs::ASSET_TYPE_LENGTH;
pub use zei::xfr::structs::{AssetType, XfrAmount, XfrAssetType};

pub const FRA_BARE_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
pub const FRA_ASSET_TYPE: XfrAssetType = XfrAssetType::NonConfidential(FRA_BARE_ASSET_TYPE);

pub type Amount = u64;

use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssetMeta {
    pub maximum: Option<U256>,
    pub transferable: bool,
}
