#![feature(generic_associated_types)]

mod utxo;
pub use utxo::UtxoModule;

pub mod utils;

mod error;
pub use error::{Error, Result};

// pub mod utxo_module_rpc {
// include!(concat!(env!("OUT_DIR"), "/utxomodule.rs"));
// }
