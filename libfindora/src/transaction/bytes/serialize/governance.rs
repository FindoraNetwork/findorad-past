use crate::{
    governance::{CreateProposal, Proposal, VoteProposal},
    governance_capnp::{create_proposal, vote_proposal},
    Result,
};

pub fn build_create_proposal(
    proposal: &CreateProposal,
    builder: create_proposal::Builder,
) -> Result<()> {
    let mut builder = builder;

    builder.set_height(proposal.height);

    let builder = builder.init_proposal();

    let mut builder = builder.init_update_rewards();

    match &proposal.proposal {
        Proposal::UpdateRewards(e) => builder.set_code(&e.data),
    }
    Ok(())
}

pub fn build_vote_proposal(proposal: &VoteProposal, builder: vote_proposal::Builder) -> Result<()> {
    let mut builder = builder.init_id();

    builder.set_txid(proposal.id.txid.as_bytes());
    builder.set_n(proposal.id.n);

    Ok(())
}
