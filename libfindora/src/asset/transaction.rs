use primitive_types::U256;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{AssetType, XfrAssetType};

use super::XfrAmount;
use crate::{transaction, Address, Error};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssetInfo {
    pub maximum: Option<U256>,
    pub transferable: bool,
    pub asset: AssetType,
    pub owner: Address,
}

#[derive(Debug)]
pub struct AssetIssue {
    pub asset: AssetType,
    pub amount: XfrAmount,
    pub address: Address,
}

#[derive(Debug, Default)]
pub struct Transaction {
    pub define_asset: Vec<AssetInfo>,
    pub transfer_asset: Vec<AssetType>,
    pub issue_asset: Vec<AssetIssue>,
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(t: &transaction::Transaction) -> Result<Self, Self::Error> {
        let mut infos = Vec::new();
        let mut types = Vec::new();
        let mut issue = Vec::new();

        for output in &t.outputs {
            match &output.operation {
                transaction::OutputOperation::TransferAsset => {
                    if let XfrAssetType::NonConfidential(at) = output.core.asset {
                        types.push(at);
                    }
                }
                transaction::OutputOperation::DefineAsset(e) => {
                    if let XfrAssetType::NonConfidential(asset) = output.core.asset {
                        let info = AssetInfo {
                            maximum: e.maximum,
                            transferable: e.transferable,
                            asset,
                            owner: output.core.address.clone(),
                        };

                        infos.push(info);
                    } else {
                        return Err(Error::MustBeNonConfidentialAsset.into());
                    }
                }
                transaction::OutputOperation::IssueAsset => {
                    if let XfrAssetType::NonConfidential(asset) = output.core.asset {
                        let info = AssetIssue {
                            asset,
                            amount: output.core.amount.clone(),
                            address: output.core.address.clone(),
                        };

                        issue.push(info);
                    } else {
                        return Err(Error::MustBeNonConfidentialAsset.into());
                    }
                }
                _ => {}
            }
        }

        Ok(Transaction {
            define_asset: infos,
            transfer_asset: types,
            issue_asset: issue,
        })
    }
}
