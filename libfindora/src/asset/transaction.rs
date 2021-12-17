use primitive_types::U256;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{AssetType, XfrAssetType};

use crate::{transaction, Error};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssetInfo {
    pub maximum: Option<U256>,
    pub transferable: bool,
    pub asset: AssetType,
}

#[derive(Debug, Default)]
pub struct Transaction {
    pub infos: Vec<AssetInfo>,
    pub types: Vec<AssetType>,
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = Error;

    fn try_from(t: &transaction::Transaction) -> Result<Self, Self::Error> {
        let infos = Vec::new();
        let mut types = Vec::new();

        for output in &t.outputs {
            match &output.operation {
                transaction::OutputOperation::TransferAsset => {
                    if let XfrAssetType::NonConfidential(at) = output.core.asset {
                        types.push(at);
                    }
                }
                _ => {}
            }
        }

        Ok(Transaction { infos, types })
    }
}
