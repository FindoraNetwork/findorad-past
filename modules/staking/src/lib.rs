#![feature(generic_associated_types)]

// mod delegate;
// mod governance;
// mod undelegate;
// mod validator_keys;

mod transaction;
use libfindora::asset::{Amount, FRA};
pub use transaction::Transaction;

mod module;
pub use module::StakingModule;

mod error;
pub use error::{Error, Result};

pub mod utils;

pub type Power = u64;

pub struct FraStaking {
    pub pre_issue: Amount,
    pub mint_limit: Amount,
    pub validator_min_power: Power,
    pub min_delegate: Amount,
    pub max_percent_per_validator: [u32; 2],
    pub undelegate_block: i64,
}

impl FraStaking {
    pub const fn total(&self) -> Amount {
        self.pre_issue + self.mint_limit
    }

    pub const fn max_delegate(&self) -> Amount {
        self.total()
    }
}

pub const FRA_STAKING: FraStaking = FraStaking {
    pre_issue: 210_0000_0000 * FRA.units,
    mint_limit: 420 * 100_0000 * FRA.units,
    validator_min_power: 1_0000 * FRA.units,
    min_delegate: 1,
    max_percent_per_validator: [1, 5],
    undelegate_block: 5,
};
