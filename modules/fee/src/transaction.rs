use libfindora::{asset::{Amount, FRA_ASSET_TYPE, XfrAmount}, Address};

use crate::Error;

pub const FRA_FEE_AMOUNT: u64 = 10_000;

#[derive(Default, Debug)]
pub struct Transaction {
    pub amount: Amount,
}

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &libfindora::Transaction) -> Result<Self, Self::Error> {
        let mut amount: Amount = 0;

        for output in &tx.outputs {
            let core = &output.core;

            if core.asset == FRA_ASSET_TYPE && core.address == Address::BlockHole {
                if let XfrAmount::NonConfidential(n) = core.amount {
                    amount = amount.checked_add(n).ok_or_else(|| Error::OverflowAdd)?;
                }
            }
        }

        let fee = Transaction { amount };

        Ok(fee)
    }
}
