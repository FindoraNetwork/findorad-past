use std::convert::{TryFrom, TryInto};

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

impl TryFrom<&Transaction> for CoinbaseTransaction {
    type Error = abcf::Error;

    fn try_from(tx: &Transaction) -> Result<Self, Self::Error> {
        let mut outputs = Vec::new();

        for i in 0..tx.outputs.len() {
            let output = &tx.outputs[i];

            match &output.operation {
                OutputOperation::IssueAsset => {
                    // safety unwrap
                    outputs.push((
                        i.try_into().map_err(|e| {
                            abcf::Error::ABCIApplicationError(
                                90001,
                                format!("convert index error, {}", e),
                            )
                        })?,
                        Output {
                            core: output.core.clone(),
                            owner_memo: output.owner_memo.clone(),
                        },
                    ))
                }
                OutputOperation::Undelegate(_) => outputs.push((
                    i.try_into().map_err(|e| {
                        abcf::Error::ABCIApplicationError(
                            90001,
                            format!("convert index error, {}", e),
                        )
                    })?,
                    Output {
                        core: output.core.clone(),
                        owner_memo: output.owner_memo.clone(),
                    },
                )),
                _ => {}
            }
        }

        Ok(Self {
            txid: tx.txid.clone(),
            outputs,
        })
    }
}
