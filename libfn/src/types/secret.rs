use bip0039::{Language, Mnemonic};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrSecretKey};

use crate::Result;

use super::PublicKey;

pub struct SecretKey {
    pub key: XfrSecretKey,
}

impl SecretKey {
    pub fn to_base64(&self) -> Result<String> {
        Ok(base64::encode(&self.key.zei_to_bytes()))
    }

    pub fn from_base64(s: &str) -> Result<Self> {
        let key = base64::decode(s)?;

        Ok(SecretKey {
            key: XfrSecretKey::zei_from_bytes(&key)?,
        })
    }

    pub fn from_mnemonic(s: &str) -> Result<Self> {
        let mnemonic = Mnemonic::from_phrase_in(Language::English, s)?;
        let seed = mnemonic.to_seed("");
        let dp = DerivationPath::bip44(917, 0, 0, 0)?;
        let esk = ExtendedSecretKey::from_seed(&seed)?.derive(&dp)?;

        let key = XfrSecretKey::zei_from_bytes(&esk.secret_key.to_bytes()[..])?;
        Ok(SecretKey { key })
    }

    pub fn to_public(&self) -> PublicKey {
        PublicKey {
            key: self.key.clone().into_keypair().pub_key,
        }
    }
}
