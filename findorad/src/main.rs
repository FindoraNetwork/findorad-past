#![feature(generic_associated_types)]

mod utxo;
mod coinbase;

use libfindora::transaction::Transaction;
use sha3::Sha3_512;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "Transaction"
)]
pub struct SimpleManager {
    pub coinbase: coinbase::CoinbaseModule,
    pub utxo: utxo::UtxoModule,
}

fn main() {}
