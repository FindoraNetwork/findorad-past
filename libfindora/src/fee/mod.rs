pub mod constant;

use crate::transaction::Transaction;

pub struct FeeTransaction {
    pub amount: u64,
}

impl Default for FeeTransaction {
    fn default() -> Self {
        FeeTransaction { amount: 0 }
    }
}

impl TryFrom<&Transaction> for FeeTransaction {
    type Error = abcf::Error;

    fn try_from(_tx: &Transaction) -> Result<Self, Self::Error> {
        let fee = FeeTransaction { amount: 1 };

        Ok(fee)
    }
}
