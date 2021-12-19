use zei::xfr::sig::{XfrPublicKey, XfrSignature};

use crate::Address;

#[derive(Debug)]
pub struct FraSignature {
    pub address: Address,
    pub public_key: XfrPublicKey,
    pub signature: XfrSignature,
}

#[derive(Debug)]
pub enum Signature {
    Fra(FraSignature),
}
