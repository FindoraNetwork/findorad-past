use crate::staking::TendermintAddress;

#[derive(Debug, Clone)]
pub struct Claim {
    pub validator: TendermintAddress,
}

#[derive(Debug, Clone)]
pub struct UpdateRewardsRuleProposal {
    pub data: Vec<u8>,
}
