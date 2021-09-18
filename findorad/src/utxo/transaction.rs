//!
//! Define Utxo related transactions
//!

use crate::utxo::{AssetCode, KeyPair, OutputId, PublicKey, Signature, TxOutPut, UtxoTx};
use ruc::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FindoraTransaction {
    pub body: Option<TxBody>,
    pub issuer: Option<PublicKey>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TxBody {
    DefineAsset(DefineAssetBody),
    IssueAsset(IssueAssetBody),
    TransferAsset(TransferAssetBody),
}

impl abcf::Transaction for FindoraTransaction {}

impl Default for FindoraTransaction {
    fn default() -> Self {
        Self {
            body: None,
            issuer: None,
            signature: None,
        }
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
        self.body.map_or_else(
            || UtxoTx::default(),
            |body| match body {
                TxBody::TransferAsset(_b) => {
                    todo!()
                }
                TxBody::IssueAsset(_b) => {
                    todo!()
                }
                TxBody::DefineAsset(_) => Default::default(),
            },
        )
    }
}

impl FindoraTransaction {
    pub fn new(body: Option<TxBody>, issuer: &KeyPair) -> Self {
        let signature = issuer.get_sk_ref().sign(
            serde_json::to_string(&body).unwrap().as_bytes(),
            issuer.get_pk_ref(),
        );

        Self {
            body,
            issuer: Some(issuer.get_pk_ref().clone()),
            signature: Some(signature),
        }
    }

    pub fn sign(&mut self, issuer: &KeyPair) {
        self.signature = Some(issuer.get_sk_ref().sign(
            serde_json::to_string(&self.body).unwrap().as_bytes(),
            issuer.get_pk_ref(),
        ));
    }

    pub fn verify(&self) -> Result<()> {
        if self.issuer.is_none() || self.signature.is_none() || self.body.is_none() {
            Err(eg!("Transaction with invalid fields"))
        } else {
            // For issuer and signature, `unwrap` will be safe here
            self.issuer
                .unwrap()
                .verify(
                    serde_json::to_string(&self.body).unwrap().as_bytes(),
                    self.signature.as_ref().unwrap(),
                )
                .c(d!("Failed to verify signature"))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAssetBody {
    pub inputs: Vec<OutputId>,
    pub outputs: Vec<TxOutPut>,
}

impl TransferAssetBody {
    pub fn add_input(&mut self, id: OutputId) {
        self.inputs.push(id);
    }

    pub fn add_output(&mut self, txo: TxOutPut) {
        self.outputs.push(txo);
    }

    /// if idx is out of bounds, nothing will be removed
    pub fn remove_input(&mut self, idx: usize) {
        if idx < self.inputs.len() {
            self.inputs.remove(idx);
        }
    }

    /// if idx is out of bounds, nothing will be removed
    pub fn remove_output(&mut self, idx: usize) {
        if idx < self.outputs.len() {
            self.outputs.remove(idx);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineAssetBody {
    pub code: Option<AssetCode>,
    pub issuer: PublicKey,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueAssetBody {
    pub units: u64,
    pub issuer: PublicKey,
}
