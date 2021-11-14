use crate::transaction::Transaction;

pub struct Fee {
    pub amount: u64,
}

impl TryFrom<&Transaction> for Fee {
    type Error = abcf::Error;

    fn try_from(_tx: &Transaction) -> Result<Self, Self::Error> {
        let fee = Fee {
            amount: 1,
        };

        Ok(fee)
    }
}

