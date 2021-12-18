mod transaction;
pub use transaction::{Transaction, AssetInfo};

pub mod meta;

pub use zei::xfr::structs::AssetType;
use zei::xfr::structs::{XfrAssetType, ASSET_TYPE_LENGTH};

pub const FRA_BARE_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
pub const FRA_ASSET_TYPE: XfrAssetType = XfrAssetType::NonConfidential(FRA_BARE_ASSET_TYPE);

pub type Amount = u64;

