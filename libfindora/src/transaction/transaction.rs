use serde::{Deserialize, Serialize};
use zei::xfr::structs::AssetTypeAndAmountProof;

use super::{Input, Output};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
}

impl abcf::Transaction for Transaction {}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
        }
    }
}

impl abcf::module::FromBytes for Transaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(bytes)?)
    }
}
