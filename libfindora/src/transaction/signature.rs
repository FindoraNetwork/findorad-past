use zei::xfr::sig::{XfrPublicKey, XfrSignature};

#[derive(Debug)]
pub struct FraSignature {
    pub public_key: XfrPublicKey,
    pub signature: XfrSignature,
}

#[derive(Debug)]
pub enum Signature {
    Fra(FraSignature),
}
