use std::convert::TryFrom;

use libfindora::utxo::{Output, OutputId};
use primitive_types::H512;
use zei::xfr::structs::AssetTypeAndAmountProof;

#[derive(Debug)]
pub struct Transaction {
    pub txid: H512,
    pub inputs: Vec<OutputId>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            txid: H512::zero(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
        }
    }
}

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &libfindora::Transaction) -> Result<Self, Self::Error> {
        let mut inputs = Vec::new();

        for input in &tx.inputs {
            match input.operation {
                libfindora::InputOperation::TransferAsset => {
                    let txid = if input.txid == H512::default() {
                        tx.txid
                    } else {
                        input.txid
                    };
                    inputs.push(OutputId {
                        txid: txid,
                        n: input.n,
                    })
                }

                _ => {}
            }
        }

        let mut outputs = Vec::new();

        for output in &tx.outputs {
            match output.operation {
                libfindora::OutputOperation::TransferAsset => outputs.push(output.core.clone()),
                _ => {}
            }
        }

        Ok(Self {
            txid: tx.txid,
            inputs,
            outputs,
            proof: tx.proof.clone(),
        })
    }
}
