//! Voting related
//!
//! deletate    -> increase voting power
//! un-delegate -> decrease voting power
//!

pub type Amount = u64;
pub type Power = u64;

/// FRA decimals
pub const FRA_DECIMALS: u8 = 6;

// tendermint address
// pub type TendermintAddr = String;

/// block height of tendermint
pub type BlockHeight = u64;

/// the maximum global power of the validator set
/// > is bounded by MaxTotalVotingPower = MaxInt64 / 8.
pub const MAX_TOTAL_POWER: Amount = Amount::MAX / 8;

/// The max vote power of any validator
/// can not exceed 20% of global power.
pub const MAX_POWER_PERCENT_PER_VALIDATOR: [u128; 2] = [1, 5];

/// How many FRA units per FRA
pub const FRA: Amount = 10_u64.pow(FRA_DECIMALS as u32);

/// Total amount of FRA-units issuance.
pub const FRA_PRE_ISSUE_AMOUNT: Amount = 210_0000_0000 * FRA;

/// Maximum allowable mint amount.
pub const MINT_AMOUNT_LIMIT: Amount = 420 * 100_0000 * FRA;

/// <Total amount of FRA-units issuance> + <token pool of CoinBase>.
pub const FRA_TOTAL_AMOUNT: Amount = FRA_PRE_ISSUE_AMOUNT + MINT_AMOUNT_LIMIT;

/// The minimum investment to become a validator through staking.
pub const STAKING_VALIDATOR_MIN_POWER: Power = 1_0000 * FRA;

/// Minimum allowable delegation amount.
pub const MIN_DELEGATION_AMOUNT: Amount = 1;
/// Maximum allowable delegation amount.
pub const MAX_DELEGATION_AMOUNT: Amount = FRA_TOTAL_AMOUNT;
