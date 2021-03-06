use std::convert::TryFrom;

use libfindora::utxo::{Output, OutputId};

use crate::Error;

#[derive(Debug, Default)]
pub struct Transaction {
    pub outputs: Vec<(OutputId, Output)>,
}

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &libfindora::Transaction) -> Result<Self, Self::Error> {
        Ok(inner(tx)?)
    }
}

fn inner(tx: &libfindora::Transaction) -> Result<Transaction, Error> {
    let mut outputs = Vec::new();

    for i in 0..tx.outputs.len() {
        let output = &tx.outputs[i];

        if let libfindora::OutputOperation::IssueAsset = output.operation {
            let output_id = OutputId {
                txid: tx.txid,
                n: i.try_into()?,
            };

            outputs.push((output_id, output.core.clone()));
        }
    }

    Ok(Transaction { outputs })
}
