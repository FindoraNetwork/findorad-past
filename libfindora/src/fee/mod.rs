pub mod constant;

use crate::transaction::Transaction;

#[derive(Default, Debug)]
pub struct FeeTransaction {
    pub amount: u64,
}

impl TryFrom<&Transaction> for FeeTransaction {
    type Error = abcf::Error;

    fn try_from(_tx: &Transaction) -> Result<Self, Self::Error> {
        let fee = FeeTransaction { amount: 1 };

        Ok(fee)
    }
}
