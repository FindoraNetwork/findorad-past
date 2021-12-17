pub mod asset;
pub mod coinbase;
pub mod fee;
pub mod rewards;
pub mod staking;
pub mod utxo;

pub mod transaction;

pub type Amount = u64;

mod address;
pub use address::Address;

pub mod error;
pub use error::{Error, Result};

pub mod transaction_capnp {
    include!(concat!(env!("OUT_DIR"), "/transaction_capnp.rs"));
}
