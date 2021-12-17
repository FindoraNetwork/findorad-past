use std::convert::TryFrom;

use crate::{transaction::Transaction};

#[derive(Debug, Default)]
pub struct CoinbaseTransaction {}

impl TryFrom<&Transaction> for CoinbaseTransaction {
    type Error = abcf::Error;

    fn try_from(_tx: &Transaction) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
