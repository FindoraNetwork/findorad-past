pub mod rpc;

use std::convert::TryFrom;
use crate::transaction::Transaction;

#[derive(Debug, Default)]
pub struct QueryTransaction {
    pub tx: Transaction,
}

impl TryFrom<&Transaction> for QueryTransaction {
    type Error = abcf::Error;

    fn try_from(tx: &Transaction) -> Result<Self, Self::Error> {
        Ok(Self{ tx:tx.clone() })
    }
}