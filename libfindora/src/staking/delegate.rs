use abcf::tm_protos::crypto;

use super::TendermintAddress;

#[derive(Debug, Clone)]
pub struct Delegate {
    pub address: TendermintAddress,
    pub validator: Option<crypto::PublicKey>,
    pub memo: Option<Vec<u8>>,
}
