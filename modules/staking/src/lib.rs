#![feature(generic_associated_types)]

mod delegate;
mod governance;
mod staking;
mod undelegate;
mod validator_keys;
mod voting;

pub use staking::staking_module_rpc;
pub use staking::StakingModule;
