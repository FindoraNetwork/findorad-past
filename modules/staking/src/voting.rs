//! Voting logics
//!
//! deletate    -> increase voting power
//! un-delegate -> decrease voting power
//!

use abcf::tm_protos::crypto;
use libfindora::staking::voting::Amount;
use libfindora::staking::voting::{
    MAX_DELEGATION_AMOUNT, MIN_DELEGATION_AMOUNT, STAKING_VALIDATOR_MIN_POWER,
};
use ruc::*;
use serde::{Deserialize, Serialize};

// crypto::PublicKey => Vec<u8>
pub fn crypto_key_2_vec(key: &crypto::PublicKey) -> Vec<u8> {
    match key.sum.as_ref().unwrap() {
        crypto::public_key::Sum::Ed25519(e) => e.to_vec(),
        crypto::public_key::Sum::Secp256k1(s) => s.to_vec(),
    }
}

/// crypto::PublicKey Wrapper
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidatorPublicKey {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
}
impl ValidatorPublicKey {
    pub fn from_crypto_publickey(cpk: &crypto::PublicKey) -> Option<Self> {
        if let Some(sum) = cpk.sum.as_ref() {
            match sum {
                crypto::public_key::Sum::Ed25519(e) => Some(Self::Ed25519(e.clone())),
                crypto::public_key::Sum::Secp256k1(s) => Some(Self::Secp256k1(s.clone())),
            }
        } else {
            None
        }
    }

    pub fn to_crypto_publickey(&self) -> Option<crypto::PublicKey> {
        match self {
            Self::Ed25519(e) => Some(crypto::PublicKey {
                sum: Some(crypto::public_key::Sum::Ed25519(e.clone())),
            }),
            Self::Secp256k1(s) => Some(crypto::PublicKey {
                sum: Some(crypto::public_key::Sum::Secp256k1(s.clone())),
            }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Voting {}

impl Voting {
    pub fn new_from_voting(voting: &Voting) -> Self {
        voting.clone()
    }

    pub fn check_delegation_amount(am: Amount, is_append: bool) -> Result<()> {
        let lowb = alt!(
            is_append,
            MIN_DELEGATION_AMOUNT,
            STAKING_VALIDATOR_MIN_POWER
        );
        if (lowb..=MAX_DELEGATION_AMOUNT).contains(&am) {
            return Ok(());
        } else {
            let msg = format!(
                "Invalid delegation amount: {} (min: {}, max: {})",
                am, lowb, MAX_DELEGATION_AMOUNT
            );
            Err(eg!(msg))
        }
    }
}
