use std::convert::TryInto;

use crate::{
    transaction::{OutputOperation, Transaction},
    utxo::Output,
};

mod rpc;
pub use rpc::{GetAssetOwnerReq, GetAssetOwnerResp};

#[derive(Debug, Default)]
pub struct CoinbaseTransaction {
    pub txid: Vec<u8>,
    pub outputs: Vec<(u32, Output)>,
}

impl From<&Transaction> for CoinbaseTransaction {
    fn from(tx: &Transaction) -> Self {
        let mut outputs = Vec::new();

        for i in 0..tx.outputs.len() {
            let output = &tx.outputs[i];
            if let OutputOperation::IssueAsset = output.operation {
                // safety unwrap
                outputs.push((
                    i.try_into().unwrap(),
                    Output {
                        core: output.core.clone(),
                        owner_memo: output.owner_memo.clone(),
                    },
                ))
            }
        }

        Self {
            txid: tx.txid.clone(),
            outputs,
        }
    }
}
