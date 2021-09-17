//!
//! Define Utxo related transactions
//!

use crate::utxo::{asset::AssetCode, KeyPair, PublicKey, Signature, TxOutPut, TxoSID, UtxoTx};
use ruc::*;
use serde::{Deserialize, Serialize};

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
pub enum Transaction {
    TransferAsset(TransferAsset),
    DefineAsset(DefineAsset),
    IssueAsset(IssueAsset),
    NOOP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAssetBody {
    pub inputs: Vec<TxoSID>,
    pub outputs: Vec<TxOutPut>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferAsset {
    pub body: TransferAssetBody,
    pub signature: Signature,
}

impl TransferAsset {
    pub fn new(body: TransferAssetBody, owner: &KeyPair) -> Self {
        let signature = owner.get_sk_ref().sign(
            serde_json::to_string(&body).unwrap().as_bytes(),
            owner.get_pk_ref(),
        );

        Self { body, signature }
    }

    pub fn add_input(&mut self, id: TxoSID) {
        self.body.inputs.push(id);
    }

    pub fn add_output(&mut self, txo: TxOutPut) {
        self.body.outputs.push(txo);
    }

    /// if idx is out of bounds, nothing will be removed
    pub fn remove_input(&mut self, idx: usize) {
        if idx < self.body.inputs.len() {
            self.body.inputs.remove(idx);
        }
    }

    /// if idx is out of bounds, nothing will be removed
    pub fn remove_output(&mut self, idx: usize) {
        if idx < self.body.outputs.len() {
            self.body.outputs.remove(idx);
        }
    }

    pub fn sign(&mut self, owner: &KeyPair) {
        self.signature = owner.get_sk_ref().sign(
            serde_json::to_string(&self.body).unwrap().as_bytes(),
            owner.get_pk_ref(),
        );
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineAssetBody {
    pub issuer: PublicKey,
    pub code: Option<AssetCode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefineAsset {
    pub body: DefineAssetBody,
    pub signature: Signature,
}

#[allow(dead_code)]
impl DefineAsset {
    pub fn new(body: DefineAssetBody, issuer: &KeyPair) -> Self {
        let signature = issuer.get_sk_ref().sign(
            serde_json::to_string(&body).unwrap().as_bytes(),
            issuer.get_pk_ref(),
        );

        Self { body, signature }
    }

    fn verify(&self) -> Result<()> {
        // 1. make sure data integrity
        // 2. check if transaction body is valid
        // 3. check signature
        self.body
            .issuer
            .verify(
                serde_json::to_string(&self.body).unwrap().as_bytes(),
                &self.signature,
            )
            .c(d!("Failed to verify singature"))
    }

    fn check_context() -> Result<()> {
        // we can do this now?
        Ok(())
    }

    fn apply() -> Result<()> {
        // yes, let's do it.
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueAssetBody {
    pub units: u64,
    pub isssuer: PublicKey,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IssueAsset {
    body: IssueAssetBody,
    signature: Signature,
}

impl IssueAsset {
    pub fn new(body: IssueAssetBody, issuer: &KeyPair) -> Self {
        let signature = issuer.get_sk_ref().sign(
            serde_json::to_string(&body).unwrap().as_bytes(),
            issuer.get_pk_ref(),
        );

        Self { body, signature }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self::NOOP
    }
}
