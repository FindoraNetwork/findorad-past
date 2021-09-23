use std::convert::TryInto;

use zei::xfr::structs::BlindAssetRecord;

use crate::transaction::{OutputOperation, Transaction};

mod rpc;
pub use rpc::{GetAssetOwnerReq, GetAssetOwnerResp};

#[derive(Debug, Default)]
pub struct CoinbaseTransacrion {
    pub txid: Vec<u8>,
    pub outputs: Vec<(u32, BlindAssetRecord)>,
}

impl From<&Transaction> for CoinbaseTransacrion {
    fn from(tx: &Transaction) -> Self {
        let mut outputs = Vec::new();

        for i in 0..tx.outputs.len() {
            let output = &tx.outputs[i];
            if let OutputOperation::IssueAsset = output.operation {
                // safety unwrap
                outputs.push((i.try_into().unwrap(), output.core.clone()))
            }
        }

        Self {
            txid: tx.txid.clone(),
            outputs,
        }
    }
}
