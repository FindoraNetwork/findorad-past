mod transaction;
pub use transaction::{AssetInfo, AssetIssue, Transaction};

pub mod meta;

use zei::xfr::structs::ASSET_TYPE_LENGTH;
pub use zei::xfr::structs::{AssetType, XfrAmount, XfrAssetType};

pub const FRA_BARE_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
pub const FRA_ASSET_TYPE: XfrAssetType = XfrAssetType::NonConfidential(FRA_BARE_ASSET_TYPE);

pub type Amount = u64;
