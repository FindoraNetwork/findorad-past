use digest::Digest;
use primitive_types::H160;
use serde::{Deserialize, Serialize};
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrPublicKey};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, PartialOrd, Ord)]
pub struct FraAddress {
    pub address: H160,
    pub public_key: XfrPublicKey,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, PartialOrd, Ord)]
pub enum Address {
    /// ETH address
    Eth(H160),
    /// Fra address
    Fra(FraAddress),
}

impl From<H160> for Address {
    fn from(e: H160) -> Self {
        Self::Eth(e)
    }
}

impl From<XfrPublicKey> for Address {
    fn from(public_key: XfrPublicKey) -> Self {
        let bytes = public_key.zei_to_bytes();
        let result = sha3::Sha3_256::digest(bytes.as_slice());
        let address = &result[0..20];
        Address::Fra(FraAddress {
            address: H160(address.try_into().expect("")),
            public_key
        })
    }
}

