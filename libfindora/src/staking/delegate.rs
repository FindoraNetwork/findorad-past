use abcf::tm_protos::crypto;

#[derive(Debug, Clone)]
pub struct Delegate {
    pub validator: crypto::PublicKey,
}
