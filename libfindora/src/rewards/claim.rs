use crate::staking::TendermintAddress;

#[derive(Debug, Clone)]
pub struct Claim {
    pub validator: TendermintAddress,
}
