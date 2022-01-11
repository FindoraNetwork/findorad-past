use libfindora::{
    asset::{Amount, FRA},
    Address,
};

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

            if let libfindora::OutputOperation::Fee = output.operation {
                if core.asset == FRA.asset_type && core.address == Address::BlockHole {
                    let n = core
                        .amount
                        .get_amount()
                        .ok_or(Error::MustBeNonConfidentialAmount)?;
                    amount = amount.checked_add(n).ok_or(Error::OverflowAdd)?;
                } else {
                    return Err(Error::MustUseFraAndBlockHole.into());
                }
            }
        }

        let fee = Transaction { amount };

        Ok(fee)
    }
}
