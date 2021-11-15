use zei::xfr::structs::{AssetType, XfrAssetType, ASSET_TYPE_LENGTH};

pub mod coinbase;
pub mod event;
pub mod fee;
pub mod rewards;
pub mod staking;
pub mod transaction;
pub mod utxo;

pub const FRA_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
pub const FRA_XFR_ASSET_TYPE: XfrAssetType = XfrAssetType::NonConfidential(FRA_ASSET_TYPE);

pub mod transaction_capnp {
    include!(concat!(env!("OUT_DIR"), "/transaction_capnp.rs"));
}
