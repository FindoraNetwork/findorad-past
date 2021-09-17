#![feature(generic_associated_types)]

use findorad::utxo::{self, transaction::FindoraTransaction};

use sha3::Sha3_512;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "FindoraTransaction"
)]
pub struct SimpleManager {
    pub utxo: utxo::UTXO,
}

fn main() {}
