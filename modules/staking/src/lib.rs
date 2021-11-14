#![feature(generic_associated_types)]

mod delegate;
mod staking;
mod undelegate;
mod validator_pubkey;
mod voting;

pub use staking::StakingModule;
