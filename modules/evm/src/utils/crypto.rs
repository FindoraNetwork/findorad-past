use libfindora::Address;

use crate::Result;

pub fn recover_address(signature: ethereum::TransactionSignature, msg: &[u8; 32]) -> Result<Address> {
    // let rid = libsecp256k1::RecoveryId::parse()

    let mut rs = [0u8; 64];
    // rs.co

    Ok(Address::default())
}
