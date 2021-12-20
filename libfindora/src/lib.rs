pub mod asset;
pub mod evm;
pub mod rewards;
pub mod staking;
pub mod utxo;

pub mod transaction;
pub use transaction::{InputOperation, OutputOperation, Transaction};

mod address;
pub use address::Address;

pub mod error;
pub use error::{Error, Result};

pub mod evm_capnp {
    include!(concat!(env!("OUT_DIR"), "/evm_capnp.rs"));
}

pub mod transaction_capnp {
    include!(concat!(env!("OUT_DIR"), "/transaction_capnp.rs"));
}
