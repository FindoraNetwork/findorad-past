use abcf::tm_protos::crypto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Delegate {
    pub address: TendermintAddress,
    pub validator: Option<ValidatorPublicKey>,
    pub memo: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
pub struct Undelegate {
    pub address: TendermintAddress,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TendermintAddress(pub [u8; 20]);

impl From<Vec<u8>> for TendermintAddress {
    fn from(e: Vec<u8>) -> Self {
        let mut r = [0u8; 20];

        r.copy_from_slice(&e);

        Self(r)
    }
}

impl From<&[u8]> for TendermintAddress {
    fn from(e: &[u8]) -> Self {
        let mut r = [0u8; 20];

        r.copy_from_slice(e);

        Self(r)
    }
}

impl From<&Vec<u8>> for TendermintAddress {
    fn from(e: &Vec<u8>) -> Self {
        let mut r = [0u8; 20];

        r.copy_from_slice(e);

        Self(r)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ValidatorPublicKey {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
    Unknown,
}

impl From<Option<crypto::PublicKey>> for ValidatorPublicKey {
    fn from(cpk: Option<crypto::PublicKey>) -> Self {
        if let Some(pk) = cpk {
            if let Some(sum) = &pk.sum {
                match sum {
                    crypto::public_key::Sum::Ed25519(e) => Self::Ed25519(e.clone()),
                    crypto::public_key::Sum::Secp256k1(s) => Self::Secp256k1(s.clone()),
                    crypto::public_key::Sum::Sr25519(_s) => Self::Unknown,
                }
            } else {
                Self::Unknown
            }
        } else {
            Self::Unknown
        }
    }
}

impl From<ValidatorPublicKey> for Option<crypto::PublicKey> {
    fn from(pk: ValidatorPublicKey) -> Self {
        match pk {
            ValidatorPublicKey::Ed25519(e) => Some(crypto::PublicKey {
                sum: Some(crypto::public_key::Sum::Ed25519(e)),
            }),
            ValidatorPublicKey::Secp256k1(s) => Some(crypto::PublicKey {
                sum: Some(crypto::public_key::Sum::Secp256k1(s)),
            }),
            ValidatorPublicKey::Unknown => None,
        }
    }
}
