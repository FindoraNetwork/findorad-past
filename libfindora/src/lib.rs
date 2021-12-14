use lazy_static::lazy_static;
use ruc::{pnk, RucResult};
use zei::{
    serialization::ZeiFromToBytes,
    xfr::{
        sig::XfrPublicKey,
        structs::{AssetType, XfrAssetType, ASSET_TYPE_LENGTH},
    },
};

pub mod coinbase;
pub mod fee;
pub mod rewards;
pub mod staking;
pub mod transaction;
pub mod utxo;

pub const FRA_ASSET_TYPE: AssetType = AssetType([0; ASSET_TYPE_LENGTH]);
pub const FRA_XFR_ASSET_TYPE: XfrAssetType = XfrAssetType::NonConfidential(FRA_ASSET_TYPE);

lazy_static! {
    /// BlackHole of Staking
    pub static ref BLACK_HOLE_PUBKEY_STAKING: XfrPublicKey = pnk!(XfrPublicKey::zei_from_bytes(&[1; ed25519_dalek::PUBLIC_KEY_LENGTH][..]));
}

pub mod transaction_capnp {
    include!(concat!(env!("OUT_DIR"), "/transaction_capnp.rs"));
}
