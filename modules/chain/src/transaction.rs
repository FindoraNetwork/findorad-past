use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct Transaction {}

impl TryFrom<&libfindora::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(_tx: &libfindora::Transaction) -> Result<Self, Self::Error> {
        Ok(Transaction {})
    }
}
