mod delegate;
use std::convert::TryFrom;

pub use delegate::Delegate;

mod undelegate;
pub use undelegate::Undelegate;

use crate::{transaction, FRA_XFR_ASSET_TYPE};
use zei::xfr::{sig::XfrPublicKey, structs::XfrAmount};

#[derive(Debug, Clone)]
pub enum Operation {
    Delegate(Delegate),
    Undelegate(Undelegate),
}

#[derive(Debug, Clone)]
pub struct StakingInfo {
    pub delegator: XfrPublicKey,
    pub amount: u64,
    pub operation: Operation,
}

pub struct Transaction {
    pub infos: Vec<StakingInfo>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            infos: Vec::new()
        }
    }
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Transaction, Self::Error> {
        macro_rules! insert_info {
            ($infos:ident, $output:ident, $op:ident, $ty:ident) => {
                if $output.core.asset_type != FRA_XFR_ASSET_TYPE {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("Undelegate asset type must be FRA"),
                    ));
                }

                let delegator = $output.core.public_key.clone();
                let amount = if let XfrAmount::NonConfidential(v) = $output.core.amount {
                    v
                } else {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("Undelegate amount must be Non-confidential"),
                    ));
                };
                let operation = Operation::$ty($op.clone());

                let info = StakingInfo {
                    delegator,
                    amount,
                    operation,
                };

                $infos.push(info);
            };
        }

        let mut infos = Vec::new();

        for output in &tx.outputs {
            match &output.operation {
                transaction::OutputOperation::Undelegate(op) => {
                    insert_info!(infos, output, op, Undelegate);
                }
                transaction::OutputOperation::Delegate(op) => {
                    insert_info!(infos, output, op, Delegate);
                }
                _ => {}
            }
        }

        Ok(Transaction { infos })
    }
}
