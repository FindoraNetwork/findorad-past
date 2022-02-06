use crate::{rewards::UpdateRewardsRuleProposal, utxo::OutputId};

#[derive(Debug, Clone)]
pub enum Proposal {
    UpdateRewards(UpdateRewardsRuleProposal),
}

#[derive(Debug, Clone)]
pub struct CreateProposal {
    pub height: i64,
    pub proposal: Proposal,
}

#[derive(Debug, Clone)]
pub struct VoteProposal {
    pub id: OutputId,
}

// #[derive(Debug, Clone)]
// pub struct CancelProposal {
//     pub id: OutputId,
// }
