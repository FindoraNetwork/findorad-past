use crate::utils::mnemonic::generate_mnemonic_custom;

use super::{Address, PublicKey, SecretKey};

use ruc::*;

pub struct Wallet {
    pub address: Address,
    pub public: PublicKey,
    pub secret: SecretKey,
}

impl Wallet {
    pub fn generate() -> Result<Self> {
        let mnemonic = generate_mnemonic_custom(24, "en")?;
        let secret = SecretKey::from_mnemonic(&mnemonic)?;
        let public = secret.to_public();
        let address = public.to_address()?;

        Ok(Wallet {
            address,
            public,
            secret,
        })
    }

    pub fn from_mnemonic(s: &str) -> Result<Self> {
        let secret = SecretKey::from_mnemonic(s)?;
        let public = secret.to_public();
        let address = public.to_address()?;

        Ok(Wallet {
            address,
            public,
            secret,
        })
    }
}
