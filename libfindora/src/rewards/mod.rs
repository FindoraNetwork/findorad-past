use zei::xfr::sig::XfrPublicKey;

mod claim;
pub use claim::Claim;

#[derive(Debug, Clone)]
pub enum Operation {
    Claim(Claim),
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub delegator: XfrPublicKey,
    pub amount: u64,
}

