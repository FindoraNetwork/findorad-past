use bip0039::{Language, Mnemonic};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use ruc::*;
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrSecretKey};

use super::PublicKey;

pub struct SecretKey {
    pub key: XfrSecretKey,
}

impl SecretKey {
    pub fn to_base64(&self) -> Result<String> {
        Ok(base64::encode(&self.key.zei_to_bytes()))
    }

    pub fn from_base64(s: &str) -> Result<Self> {
        let key = base64::decode(s).c(d!())?;

        Ok(SecretKey {
            key: XfrSecretKey::zei_from_bytes(&key)?,
        })
    }

    pub fn from_mnemonic(s: &str) -> Result<Self> {
        let mnemonic = Mnemonic::from_phrase_in(Language::English, s).c(d!())?;
        let seed = mnemonic.to_seed("");
        let dp = DerivationPath::bip44(917, 0, 0, 0).map_err(|e| eg!(e))?;
        let esk = ExtendedSecretKey::from_seed(&seed)
            .map_err(|e| eg!(e))?
            .derive(&dp)
            .map_err(|e| eg!(e))?;

        let key = XfrSecretKey::zei_from_bytes(&esk.secret_key.to_bytes()[..]).c(d!())?;
        Ok(SecretKey { key })
    }

    pub fn to_public(&self) -> PublicKey {
        PublicKey {
            key: self.key.clone().into_keypair().pub_key,
        }
    }
}
