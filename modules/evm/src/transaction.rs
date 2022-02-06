use std::convert::TryFrom;

use crate::Error;
use libfindora::{asset::XfrAmount, evm::Action, transaction::OutputOperation, Address};

#[derive(Debug, Default)]
pub struct EvmTransaction {
    pub chain_id: u64,
    pub from: Address,
    pub to: Address,
    pub nonce: u64,
    pub amount: u64,
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

    for output in &tx.outputs {
        if let OutputOperation::EvmCall(e) = &output.operation {
            if let XfrAmount::NonConfidential(amount) = output.core.amount {
                let chain_id = e.chain_id;
                let from = e.caller.clone();
                let to = output.core.address.clone();
                let nonce = e.nonce;
                let data = e.data.clone();
                let action = e.action.clone();
                let gas_limit = e.gas_limit;

                txs.push(EvmTransaction {
                    chain_id,
                    from,
                    to,
                    nonce,
                    amount,
                    data,
                    action,
                    gas_limit,
                })
            }
        }
    }

    Ok(Transaction { txs })
}
