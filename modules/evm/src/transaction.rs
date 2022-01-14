use std::convert::TryFrom;

use crate::Error;
use libfindora::{
    evm::Action,
    transaction::{InputOperation, Memo, OutputOperation},
    utxo::OutputId,
    Address,
};

#[derive(Debug, Default)]
pub struct EvmTransaction {
    pub chain_id: Option<u64>,
    pub from: Option<Address>,
    pub from_output: Option<OutputId>,
    pub to: Address,
    pub nonce: u64,
    pub data: Vec<u8>,
    pub action: Action,
    pub gas_limit: u64,
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

    // verify ethereum memo.
    for memo in &tx.memos {
        match memo {
            Memo::Ethereum(bytes) => {
                // verify tx signature.

                // verify utxo type.

                // parse ethereum.
                //                 let rlp = Rlp::new(&bytes.tx);
                // match ethereum::TransactionV2::decode(&rlp)? {
                //     TransactionV2::Legacy(e) => {},
                //     _ => return Err(Error::OnlySupportLegacyTransaction);
                //                 }
            }
        }
    }

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
                //                 let nonce = e.nonce;
                // let data = e.data.clone();
                // let action = e.action.clone();
                //
                // let etx = EvmTransaction {
                //     from,
                //     to,
                //     nonce,
                //     data,
                //     action,
                // };
                //
                //                 txs.push(etx);
            } else {
                return Err(Error::OutputOperationMustBeEvm);
            }
        }
    }

    Ok(Transaction { txs })
}
