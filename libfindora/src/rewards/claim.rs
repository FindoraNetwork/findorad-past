use abcf::tm_protos::crypto;

#[derive(Debug, Clone)]
pub struct Claim {
    pub validator: crypto::PublicKey,
}
