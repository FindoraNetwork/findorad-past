use serde::{Serialize, Deserialize};

use crate::utxo::UtxoTx;

#[derive(Serialize, Deserialize)]
pub struct FindoraTransaction {
    pub v: u64,
}

impl abcf::Transaction for FindoraTransaction {}

impl Default for FindoraTransaction {
    fn default() -> Self {
        Self { v: 0 }
    }
}

impl abcf::module::FromBytes for FindoraTransaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(bytes)?)
    }
}

impl Into<UtxoTx> for FindoraTransaction {
    fn into(self) -> UtxoTx {
        UtxoTx::default()
    }
}
