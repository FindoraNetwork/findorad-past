use crate::staking::TendermintAddress;

#[derive(Debug, Clone)]
pub struct Undelegate {
    pub address: TendermintAddress,
}
