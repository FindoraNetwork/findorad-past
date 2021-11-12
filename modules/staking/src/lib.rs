#![feature(generic_associated_types)]

mod voting;
mod validator_pubkey;
mod staking;
mod delegate;
mod undelegate;

pub use staking::StakingModule;