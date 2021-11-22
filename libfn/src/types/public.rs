use bech32::{FromBase32, ToBase32};
use primitive_types::H160;
use ruc::*;
use sha3::Digest;
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrPublicKey};

use super::Address;

pub struct PublicKey {
    pub key: XfrPublicKey,
}

impl PublicKey {
    pub fn to_bech32(&self) -> Result<String> {
        bech32::encode(
            "fra",
            self.key.zei_to_bytes().to_base32(),
            bech32::Variant::Bech32,
        )
        .c(d!())
    }

    pub fn to_base64(&self) -> Result<String> {
        Ok(base64::encode(&self.key.zei_to_bytes()))
    }

    pub fn from_bech32(s: &str) -> Result<Self> {
        let (prefix, key, _) = bech32::decode(s).c(d!())?;

        if prefix != "fra" {
            return Err(eg!("fra public key must be fra"));
        }

        let key = Vec::from_base32(&key).c(d!())?;

        Ok(PublicKey {
            key: XfrPublicKey::zei_from_bytes(&key)?,
        })
    }

    pub fn from_base64(s: &str) -> Result<Self> {
        let key = base64::decode(s).c(d!())?;

        Ok(PublicKey {
            key: XfrPublicKey::zei_from_bytes(&key)?,
        })
    }

    pub fn to_address(&self) -> Result<Address> {
        let bytes = self.key.zei_to_bytes();
        let result = sha3::Sha3_256::digest(bytes.as_slice());
        let address = &result[0..20];
        Ok(Address {
            address: H160::from_slice(address),
        })
    }
}
