use zei::xfr::structs::BlindAssetRecord;

use crate::transaction::{OutputOperation, Transaction};

mod rpc;
pub use rpc::{GetAssetOwnerReq, GetAssetOwnerResp};

#[derive(Debug, Default)]
pub struct CoinbaseTransacrion {
    pub outputs: Vec<BlindAssetRecord>,
}

impl From<&Transaction> for CoinbaseTransacrion {
    fn from(tx: &Transaction) -> Self {
        let mut outputs = Vec::new();

        for output in &tx.outputs {
            if let OutputOperation::IssueAsset = output.operation {
                outputs.push(output.core.clone())
            }
        }

        Self { outputs }
    }
}
