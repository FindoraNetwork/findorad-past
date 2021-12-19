use bech32::{FromBase32, ToBase32};
use primitive_types::H160;
use ruc::*;

pub struct Address {
    pub address: H160,
}

impl Address {
    pub fn to_eth(&self) -> Result<String> {
        Ok(format!("0x{}", hex::encode(self.address)))
    }

    pub fn to_bech32(&self) -> Result<String> {
        bech32::encode("fra1", self.address.0.to_base32(), bech32::Variant::Bech32).c(d!())
    }

    pub fn to_base64(&self) -> Result<String> {
        Ok(base64::encode(&self.address.0))
    }

    pub fn from_eth(s: &str) -> Result<Self> {
        if &s[..2] != "0x" {
            return Err(eg!("eth must start with 0x"));
        }

        let bytes = &s[2..];

        if bytes.len() != 40 {
            return Err(eg!("eth length must be 40"));
        }

        let address = hex::decode(bytes).c(d!())?;

        Ok(Address {
            address: H160::from_slice(&address),
        })
    }

    pub fn from_bech32(s: &str) -> Result<Self> {
        let (prefix, add32, _) = bech32::decode(s).c(d!())?;

        if prefix != "fra1" {
            return Err(eg!("prefix of address bech must be fra1"));
        }

        let address_inner = Vec::from_base32(&add32).c(d!())?;

        let address = H160::from_slice(&address_inner);

        Ok(Address { address })
    }

    pub fn from_base64(s: &str) -> Result<Self> {
        let address = base64::decode(s).c(d!())?;

        let address = H160::from_slice(&address);

        Ok(Address { address })
    }
}
