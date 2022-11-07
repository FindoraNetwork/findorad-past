use primitive_types::H512;

use crate::{
    governance::{CreateProposal, Proposal, VoteProposal},
    governance_capnp::{create_proposal, vote_proposal},
    rewards::UpdateRewardsRuleProposal,
    utxo::OutputId,
    Result,
};

pub fn from_create_proposal(reader: create_proposal::Reader) -> Result<CreateProposal> {
    let height = reader.get_height();

    let reader = reader.get_proposal()?.get_update_rewards()?;
    let code = reader.get_code()?.to_vec();

    let proposal = Proposal::UpdateRewards(UpdateRewardsRuleProposal { data: code });

    Ok(CreateProposal { height, proposal })
}

pub fn from_vote_proposal(reader: vote_proposal::Reader) -> Result<VoteProposal> {
    let txid = H512::from_slice(reader.get_id()?.get_txid()?);
    let n = reader.get_id()?.get_n();

    Ok(VoteProposal {
        id: OutputId { txid, n },
    })
}
