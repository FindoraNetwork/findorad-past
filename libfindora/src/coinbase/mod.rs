use zei::xfr::structs::BlindAssetRecord;

use crate::transaction::{Transaction, OutputOperation};

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

        Self {
            outputs,
        }
    }
}
