mod transaction;
pub use transaction::Transaction;

pub mod meta;

use zei::xfr::structs::{AssetType, XfrAssetType, ASSET_TYPE_LENGTH};

pub const FRA_BARE_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
pub const FRA_ASSET_TYPE: XfrAssetType = XfrAssetType::NonConfidential(FRA_BARE_ASSET_TYPE);
