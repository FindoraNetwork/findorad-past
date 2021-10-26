mod delegate;
pub use delegate::Delegate;

mod undelegate;
pub use undelegate::Undelegate;

use zei::xfr::sig::XfrPublicKey;
use crate::transaction;

#[derive(Debug, Clone)]
pub enum Operation {
    Delegate(Delegate),
    Undelegate(Undelegate),
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub delegator: XfrPublicKey,
    pub amount: u64,
    pub operation: Operation,
}

impl From<&transaction::Transaction> for Transaction {
    fn from(tx: &transaction::Transaction) -> Self {
        // unwrap in this method will remove when next version of abcf.

        let delegator = None;
        let amount = None;
        let operation = None;

        for output in tx.outputs {
            match output.operation {
                transaction::OutputOperation::Undelegate(op) => {
                    delegator = Some(output.core.public_key.clone());
                    // if let
                },
                _ => {}
            }
        }

        Transaction {
            delegator: delegator.unwrap(),
            amount: amount.unwrap(),
            operation: operation.unwrap(),
        }
    }
}

