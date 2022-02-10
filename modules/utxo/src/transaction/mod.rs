mod validate;
pub use validate::ValidateTransaction;

use std::convert::TryFrom;

use libfindora::{
    utxo::{Output, OutputId},
    Address,
};
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
                    let txid = if input.txid == H512::zero() {
                        tx.txid
                    } else {
                        input.txid
                    };
                    inputs.push(OutputId { txid, n: input.n })
                }
            }
        }

        let mut outputs = Vec::new();

        for output in &tx.outputs {
            match output.operation {
                libfindora::OutputOperation::TransferAsset => outputs.push(output.core.clone()),
                libfindora::OutputOperation::Fee => outputs.push(output.core.clone()),
                libfindora::OutputOperation::Delegate(_) => {
                    // Here you need to do something, change the address in the output to blockhole,
                    // previously this address was delegator, if no change is made here,
                    // this money will be treated as a reasonable utxo.
                    let mut output_new = output.core.clone();
                    output_new.address = Address::blockhole();
                    outputs.push(output_new);
                }
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
