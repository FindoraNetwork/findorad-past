pub mod constant;

use zei::xfr::structs::XfrAmount;

use crate::{transaction, Error, asset::{Amount, FRA_ASSET_TYPE}, Address};

#[derive(Default, Debug)]
pub struct Transaction {
    pub amount: Amount,
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Self, Self::Error> {

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
