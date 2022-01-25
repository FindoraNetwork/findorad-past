use bech32::{FromBase32, ToBase32};
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrPublicKey};

use crate::{Error, Result};

use super::Address;

pub struct PublicKey {
    pub key: XfrPublicKey,
}

impl PublicKey {
    pub fn to_bech32(&self) -> Result<String> {
        Ok(bech32::encode(
            "fra",
            self.key.zei_to_bytes().to_base32(),
            bech32::Variant::Bech32,
        )?)
    }

    pub fn to_base64(&self) -> Result<String> {
        Ok(base64::encode_config(
            &self.key.zei_to_bytes(),
            base64::URL_SAFE,
        ))
    }

    pub fn from_bech32(s: &str) -> Result<Self> {
        let (prefix, key, _) = bech32::decode(s)?;

        if prefix != "fra" {
            return Err(Error::FraAddressFormatError);
        }

        let key = Vec::from_base32(&key)?;

        Ok(PublicKey {
            key: XfrPublicKey::zei_from_bytes(&key)?,
        })
    }

    pub fn from_base64(s: &str) -> Result<Self> {
        let key = base64::decode_config(s, base64::URL_SAFE)?;

        Ok(PublicKey {
            key: XfrPublicKey::zei_from_bytes(&key)?,
        })
    }

    pub fn to_address(&self) -> Result<Address> {
        let address = libfindora::Address::from(self.key);

        Ok(Address { address: address.0 })
    }
}
