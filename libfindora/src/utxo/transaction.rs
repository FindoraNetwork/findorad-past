use crate::transaction::Transaction;
use zei::xfr::structs::{AssetTypeAndAmountProof, BlindAssetRecord};

pub struct Input {
    pub txid: Vec<u8>,
    pub n: usize,
}

pub struct Output {
    pub core: BlindAssetRecord,
}

pub struct UtxoTransacrion {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
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

        let outputs = tx
            .outputs
            .iter()
            .map(|o| Output {
                core: o.core.clone(),
            })
            .collect();

        Self {
            inputs,
            outputs,
            proof: tx.proof.clone(),
        }
    }
}
