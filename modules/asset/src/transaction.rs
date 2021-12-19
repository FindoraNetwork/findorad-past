use libfindora::{
    asset::{AssetType, XfrAmount, XfrAssetType},
    Address,
};
use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::Error;

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

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(t: &libfindora::Transaction) -> Result<Self, Self::Error> {
        let mut infos = Vec::new();
        let mut types = Vec::new();
        let mut issue = Vec::new();

        for output in &t.outputs {
            match &output.operation {
                libfindora::OutputOperation::TransferAsset => {
                    if let XfrAssetType::NonConfidential(at) = output.core.asset {
                        types.push(at);
                    }
                }
                libfindora::OutputOperation::DefineAsset(e) => {
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
                libfindora::OutputOperation::IssueAsset => {
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
