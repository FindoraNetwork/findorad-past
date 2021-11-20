use std::convert::{TryFrom, TryInto};

use crate::{
    transaction::{OutputOperation, Transaction},
    utxo::Output,
};

mod rpc;
use primitive_types::H512;
pub use rpc::{GetAssetOwnerReq, GetAssetOwnerResp};

#[derive(Debug, Default)]
pub struct CoinbaseTransaction {
    pub txid: H512,
    pub outputs: Vec<(u32, Output)>,
}

impl TryFrom<&Transaction> for CoinbaseTransaction {
    type Error = abcf::Error;

    fn try_from(tx: &Transaction) -> Result<Self, Self::Error> {
        let mut outputs = Vec::new();

        for i in 0..tx.outputs.len() {
            let output = &tx.outputs[i];
            if let OutputOperation::IssueAsset = output.operation {
                // safety unwrap
                outputs.push((
                    i.try_into().map_err(|e| {
                        abcf::Error::ABCIApplicationError(
                            90001,
                            format!("convert index error, {}", e),
                        )
                    })?,
                    output.core.clone(),
                ))
            }
        }

        Ok(Self {
            txid: tx.txid.clone(),
            outputs,
        })
    }
}
