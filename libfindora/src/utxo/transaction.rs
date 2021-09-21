use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{AssetTypeAndAmountProof, BlindAssetRecord};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    pub txid: Vec<u8>,
    pub n: u32,
}

#[derive(Debug)]
pub struct UtxoTransacrion {
    pub inputs: Vec<Input>,
    pub outputs: Vec<BlindAssetRecord>,
    pub proof: AssetTypeAndAmountProof,
}

impl Default for UtxoTransacrion {
    fn default() -> Self {
        UtxoTransacrion {
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
        }
    }
}

impl From<&Transaction> for UtxoTransacrion {
    fn from(tx: &Transaction) -> Self {
        let inputs = tx
            .inputs
            .iter()
            .map(|i| Input {
                txid: i.txid.clone(),
                n: i.n,
            })
            .collect();

        let outputs = tx.outputs.iter().map(|o| o.core.clone()).collect();

        Self {
            inputs,
            outputs,
            proof: tx.proof.clone(),
        }
    }
}
