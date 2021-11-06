use std::convert::TryFrom;

use zei::xfr::{sig::XfrPublicKey, structs::XfrAmount};

mod claim;
pub use claim::Claim;

use crate::{transaction, FRA_XFR_ASSET_TYPE};

#[derive(Debug, Clone)]
pub enum Operation {
    Claim(Claim),
}

#[derive(Debug, Clone)]
pub struct RewardInfo {
    pub delegator: XfrPublicKey,
    pub amount: u64,
    pub operation: Operation,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub infos: Vec<RewardInfo>,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction { infos: Vec::new() }
    }
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Transaction, Self::Error> {
        let mut infos = Vec::new();

        for output in &tx.outputs {
            match &output.operation {
                transaction::OutputOperation::ClaimReward(op) => {
                    if output.core.asset_type != FRA_XFR_ASSET_TYPE {
                        return Err(abcf::Error::ABCIApplicationError(
                            90001,
                            String::from("Undelegate asset type must be FRA"),
                        ));
                    }

                    let delegator = output.core.public_key.clone();
                    let amount = if let XfrAmount::NonConfidential(v) = output.core.amount {
                        v
                    } else {
                        return Err(abcf::Error::ABCIApplicationError(
                            90001,
                            String::from("Undelegate amount must be Non-confidential"),
                        ));
                    };
                    let operation = Operation::Claim(op.clone());

                    let info = RewardInfo {
                        delegator,
                        amount,
                        operation,
                    };

                    infos.push(info);
                }
                _ => {}
            }
        }

        Ok(Transaction { infos })
    }
}
