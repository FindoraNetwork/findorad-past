use libfindora::{
    asset::{Amount, FRA},
    staking::{Delegate, Undelegate},
    Address,
};
use std::convert::TryFrom;

use crate::Error;

#[derive(Debug, Clone)]
pub enum Operation {
    Delegate(Delegate),
    Undelegate(Undelegate),
}

#[derive(Debug, Clone)]
pub struct StakingInfo {
    pub delegator: Address,
    pub amount: Amount,
    pub operation: Operation,
}

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub infos: Vec<StakingInfo>,
}

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &libfindora::Transaction) -> Result<Transaction, Self::Error> {
        let mut infos = Vec::new();
        let mut outputs = Vec::new();

        // since taking comes first, you must determine if the operation is related to delegation before performing the conversion.
        for output in tx.outputs.iter() {
            match &output.operation {
                libfindora::OutputOperation::Delegate(_)
                | libfindora::OutputOperation::Undelegate(_) => outputs.push(output.clone()),
                _ => continue,
            }
        }

        if !outputs.is_empty() {
            for output in outputs.iter() {
                if output.core.asset != FRA.asset_type {
                    return Err(Error::MustBeFra.into());
                }

                let amount = output
                    .core
                    .amount
                    .get_amount()
                    .ok_or(Error::MustBeNonConfidentialAmount)?;

                let delegator = output.core.address.clone();

                let op = match &output.operation {
                    libfindora::OutputOperation::Delegate(op) => Operation::Delegate(op.clone()),
                    libfindora::OutputOperation::Undelegate(op) => {
                        Operation::Undelegate(op.clone())
                    }
                    _ => {
                        return Err(abcf::Error::ABCIApplicationError(
                            90009,
                            "staking module internal errors".to_string(),
                        ))
                    }
                };
                let info = StakingInfo {
                    amount,
                    delegator,
                    operation: op,
                };
                infos.push(info);
            }
        }

        Ok(Transaction { infos })
    }
}
