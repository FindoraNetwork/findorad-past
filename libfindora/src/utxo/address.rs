use primitive_types::H160;
use serde::{Deserialize, Serialize};
use zei::xfr::sig::XfrPublicKey;

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
