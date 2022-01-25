use digest::Digest;
use primitive_types::{H160, H256};
use serde::{Deserialize, Serialize};
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrPublicKey};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, PartialOrd, Ord, Default)]
pub struct Address(pub H160);

impl Address {
    pub fn blockhole() -> Self {
        Self(H160::zero())
    }

    pub fn is_blockhole(&self) -> bool {
        self.0.is_zero()
    }
}

impl From<&[u8]> for Address {
    fn from(e: &[u8]) -> Self {
        Self(H160::from_slice(e))
    }
}

impl From<H160> for Address {
    fn from(e: H160) -> Self {
        Self(e)
    }
}

impl From<XfrPublicKey> for Address {
    fn from(public_key: XfrPublicKey) -> Self {
        let bytes = public_key.zei_to_bytes();
        let result = sha3::Sha3_256::digest(bytes.as_slice());
        let h256 = H256::from_slice(&result);
        Self(H160::from(h256))
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
