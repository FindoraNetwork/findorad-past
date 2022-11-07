use std::convert::TryFrom;

use libfindora::{
    governance::{CreateProposal, VoteProposal},
    transaction::{self, OutputOperation},
};

#[derive(Debug, Clone, Default)]
pub struct Transaction {
    pub creates: Vec<CreateProposal>,
    pub votes: Vec<VoteProposal>,
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Transaction, Self::Error> {
        let mut creates = Vec::new();
        let mut votes = Vec::new();

        for output in &tx.outputs {
            match &output.operation {
                OutputOperation::CreateProposal(p) => creates.push(p.clone()),
                OutputOperation::VoteProposal(p) => votes.push(p.clone()),
                _ => {}
            }
        }

        Ok(Transaction { creates, votes })
    }
}
