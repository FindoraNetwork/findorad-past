use std::convert::TryFrom;

use libfindora::{
    evm::Action,
    transaction::{InputOperation, OutputOperation},
    utxo::OutputId,
    Address,
};

use crate::Error;

#[derive(Debug, Default)]
pub struct EvmTransaction {
    pub from: OutputId,
    pub to: Address,
    pub nonce: u64,
    pub data: Vec<u8>,
    pub action: Action,
}

#[derive(Debug, Default)]
pub struct Transaction {
    pub txs: Vec<EvmTransaction>,
}

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &libfindora::Transaction) -> Result<Self, Self::Error> {
        Ok(inner(tx)?)
    }
}

fn inner(tx: &libfindora::Transaction) -> Result<Transaction, Error> {
    let mut txs = Vec::new();

    for input in &tx.inputs {
        if let InputOperation::EvmCall(a) = &input.operation {
            let from = OutputId {
                txid: input.txid,
                n: input.n,
            };

            let n: usize = a.n.try_into()?;

            let output = tx.outputs.get(n).ok_or_else(|| Error::NoOutputIndex)?;
            let to = output.core.address.clone();

            if let OutputOperation::EvmCall(e) = &output.operation {
                let nonce = e.nonce;
                let data = e.data.clone();
                let action = e.action.clone();

                let etx = EvmTransaction {
                    from,
                    to,
                    nonce,
                    data,
                    action,
                };

                txs.push(etx);
            } else {
                return Err(Error::OutputOperationMustBeEvm);
            }
        }
    }

    Ok(Transaction { txs })
}
