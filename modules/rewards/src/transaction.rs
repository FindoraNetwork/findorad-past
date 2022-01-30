use std::convert::TryFrom;

use libfindora::{Address, staking::TendermintAddress, transaction, asset::FRA};
use zei::xfr::structs::XfrAmount;

#[derive(Debug, Clone)]
pub struct RewardInfo {
    pub delegator: Address,
    pub amount: u64,
    pub validator: TendermintAddress,
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub infos: Vec<RewardInfo>,
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Transaction, Self::Error> {
        let mut infos = Vec::new();

        for output in &tx.outputs {
            if let transaction::OutputOperation::ClaimReward(op) = &output.operation {
                if output.core.asset != FRA.asset_type {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("Undelegate asset type must be FRA"),
                    ));
                }

                let delegator = output.core.address.clone();
                let amount = if let XfrAmount::NonConfidential(v) = output.core.amount {
                    v
                } else {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("Undelegate amount must be Non-confidential"),
                    ));
                };
                let validator = op.validator.clone();

                let info = RewardInfo {
                    delegator,
                    amount,
                    validator,
                };

                infos.push(info);
            }
        }

        Ok(Transaction { infos })
    }
}
