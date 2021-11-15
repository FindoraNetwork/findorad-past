use abcf::tm_protos::crypto;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use zei::xfr::sig::XfrPublicKey;

pub type ValidatorPubKeyPair = (ValidatorPublicKey, XfrPublicKey);

/// validator addr from ValidatorPublicKey sha256[0..20]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ValidatorAddr {
    pub addr: Vec<u8>,
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

    pub fn get_bytes(&self) -> Vec<u8> {
        match self {
            ValidatorPublicKey::Ed25519(v) => v.clone(),
            ValidatorPublicKey::Secp256k1(v) => v.clone(),
        }
    }

    pub fn to_validator_addr(&self) -> ValidatorAddr {
        let mut hasher = Sha3_256::new();

        hasher.update(self.get_bytes());

        let result = hasher.finalize()[0..20].to_vec();

        ValidatorAddr { addr: result }
    }
}