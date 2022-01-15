use std::{collections::BTreeMap, convert::TryFrom};

use crate::Error;
use libfindora::{
    asset::XfrAmount, evm::Action, transaction::OutputOperation, utxo::OutputId, Address,
};

#[derive(Debug, Default)]
pub struct EvmTransaction {
    pub chain_id: u64,
    pub from: Option<Address>,
    pub from_output: Vec<OutputId>,
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

    let mut out_indexes: BTreeMap<u32, Vec<OutputId>> = BTreeMap::new();

    for input in &tx.inputs {
        if let Some(value) = out_indexes.get_mut(&input.n) {
            value.push(OutputId {
                txid: input.txid,
                n: input.n,
            });
        } else {
            let mut value = Vec::new();
            value.push(OutputId {
                txid: input.txid,
                n: input.n,
            });
            out_indexes.insert(input.n, value);
        }
    }

    for index in 0..tx.outputs.len() {
        let output = &tx.outputs[index];

        if let OutputOperation::EvmCall(e) = &output.operation {
            let nonce = e.nonce;
            let data = e.data.clone();
            let action = e.action.clone();
            let gas_limit = e.gas_limit;
            let chain_id = e.chain_id;
            let to = output.core.address.clone();

            let index: u32 = index.try_into()?;

            let from_output = if let Some(v) = out_indexes.get(&index) {
                v.clone()
            } else {
                return Err(Error::NoOutputIndex);
            };

            let amount = if let XfrAmount::NonConfidential(amount) = output.core.amount {
                amount
            } else {
                return Err(Error::AmountTypeMustBeNonConfidential);
            };

            let etx = EvmTransaction {
                from: None,
                from_output,
                to,
                nonce,
                amount,
                data,
                action,
                gas_limit,
                chain_id,
            };

            // Verify signature and arguments for evm.

            txs.push(etx);
        }
    }

    Ok(Transaction { txs })
}
