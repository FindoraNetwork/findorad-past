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

        for output in &tx.outputs {
            if output.core.asset == FRA.asset_type {
                return Err(Error::MustBeFra.into());
            }

            let amount = output
                .core
                .amount
                .get_amount()
                .ok_or(Error::MustBeNonConfidentialAmount)?;

            let delegator = output.core.address.clone();

            match &output.operation {
                libfindora::OutputOperation::Delegate(op) => {
                    let info = StakingInfo {
                        amount,
                        delegator,
                        operation: Operation::Delegate(op.clone()),
                    };

                    infos.push(info);
                }
                libfindora::OutputOperation::Undelegate(op) => {
                    let info = StakingInfo {
                        amount,
                        delegator,
                        operation: Operation::Undelegate(op.clone()),
                    };

                    infos.push(info);
                }
                _ => {}
            }
        }

        Ok(Transaction { infos })
    }
}
