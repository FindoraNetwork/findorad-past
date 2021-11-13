#![feature(generic_associated_types)]

mod delegate;
mod governance;
mod staking;
mod undelegate;
mod validator_module;
mod voting;

pub use staking::StakingModule;
