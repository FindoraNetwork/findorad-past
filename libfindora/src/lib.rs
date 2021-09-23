pub mod coinbase;
pub mod transaction;
pub mod utxo;

pub mod transaction_capnp {
    include!(concat!(env!("OUT_DIR"), "/transaction_capnp.rs"));
}
