use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{AssetTypeAndAmountProof, BlindAssetRecord, OwnerMemo};

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    pub txid: Vec<u8>,
    pub n: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Output {
    pub core: BlindAssetRecord,
    pub owner_memo: Option<OwnerMemo>,
}

#[derive(Debug)]
pub struct UtxoTransacrion {
    pub txid: Vec<u8>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
}

impl Default for UtxoTransacrion {
    fn default() -> Self {
        UtxoTransacrion {
            txid: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
        }
    }
}

impl From<&Transaction> for UtxoTransacrion {
    fn from(tx: &Transaction) -> Self {
        let mut inputs = Vec::new();

        for input in &tx.inputs {
            if input.txid == Vec::<u8>::new() {
                inputs.push(Input {
                    txid: tx.txid.clone(),
                    n: input.n,
                })
            } else {
                inputs.push(Input {
                    txid: input.txid.clone(),
                    n: input.n,
                })
            }
        }

        let mut outputs = Vec::new();

        for output in &tx.outputs {
            outputs.push(Output {
                core: output.core.clone(),
                owner_memo: output.owner_memo.clone(),
            });
        }

        Self {
            txid: tx.txid.clone(),
            inputs,
            outputs,
            proof: tx.proof.clone(),
        }
    }
}
