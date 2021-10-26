use abcf::tm_protos::crypto;

#[derive(Debug, Clone)]
pub struct Undelegate {
    pub validator: crypto::PublicKey,
}
