use capnp::{message::ReaderOptions, serialize::read_message, serialize_packed};
use serde::{Deserialize, Serialize};
use zei::xfr::structs::AssetTypeAndAmountProof;

use crate::transaction_capnp;

use super::{Input, Output};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub txid: Vec<u8>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
}

impl abcf::Transaction for Transaction {}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            txid: Vec::new(),
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
        let reader = read_message(bytes, ReaderOptions::new())
            .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

        let root = reader
            .get_root::<transaction_capnp::transaction::Reader>()
            .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

        let txid = root.get_txid();

        Ok(Self::default())
    }
}
