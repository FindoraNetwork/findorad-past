use abcf::tm_protos::crypto;
use zei::xfr::sig::XfrPublicKey;

pub enum Operation {
    Delegate,
    Undelegate,
}

pub struct Transaction {
    pub validator: crypto::PublicKey,
    pub delegator: XfrPublicKey,
    pub amount: u64,
    pub operation: Operation,
}
