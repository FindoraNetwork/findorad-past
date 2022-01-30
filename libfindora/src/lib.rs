pub mod asset;
pub mod evm;
pub mod governance;
pub mod rewards;
pub mod staking;
pub mod utxo;

pub mod transaction;
pub use transaction::{InputOperation, OutputOperation, Transaction};

mod address;
pub use address::Address;

pub mod error;
pub use error::{Error, Result};

pub mod address_capnp {
    include!(concat!(env!("OUT_DIR"), "/address_capnp.rs"));
}

pub mod evm_capnp {
    include!(concat!(env!("OUT_DIR"), "/evm_capnp.rs"));
}

pub mod memo_capnp {
    include!(concat!(env!("OUT_DIR"), "/memo_capnp.rs"));
}

pub mod input_capnp {
    include!(concat!(env!("OUT_DIR"), "/input_capnp.rs"));
}

pub mod output_capnp {
    include!(concat!(env!("OUT_DIR"), "/output_capnp.rs"));
}

pub mod transaction_capnp {
    include!(concat!(env!("OUT_DIR"), "/transaction_capnp.rs"));
}

pub mod governance_capnp {
    include!(concat!(env!("OUT_DIR"), "/governance_capnp.rs"));
}
