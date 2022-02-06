use std::convert::TryFrom;

use libfindora::transaction;

#[derive(Debug, Clone, Default)]
pub struct Transaction {}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Transaction, Self::Error> {
        Ok(Transaction {})
    }
}
