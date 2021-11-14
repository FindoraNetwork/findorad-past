use ethereum::TransactionV2;

pub struct EthereumTransaction {
    pub tx: Option<TransactionV2>,
}

impl Default for EthereumTransaction {
    fn default() -> Self {
        Self {
            tx: None,
        }
    }
}

