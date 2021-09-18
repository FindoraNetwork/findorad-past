//!
//!

mod txbuilder;

use crate::utxo::{Amount, AssetCode, KeyPair, PublicKey};
use ruc::*;

pub fn create_new_asset(_owner: &KeyPair) -> Result<()> {
    // 1. generate random asset code
    // create_new_asset_with_code(owner, code)
    Ok(())
}

pub fn create_new_asset_with_code(_owner: &KeyPair, _code: &str) -> Result<()> {
    // 1. query if asset exists
    // 2. construct transaction
    // 3. check result
    Ok(())
}

pub fn issue_asset(_issuer: &KeyPair, _asset: AssetCode, _am: Amount) -> Result<()> {
    // 1. query if asset exists
    // 2. check if issuer owns this asset
    // 2. construct transaction
    // 3. check result
    Ok(())
}

pub fn tranfer_asset(
    _owner: &KeyPair,
    _asset: AssetCode,
    _ams: &[Amount],
    _receivers: &[PublicKey],
) -> Result<()> {
    // 1. query if asset exists
    // 2. check if owner owns this asset
    // 3. check custom asset balance
    // 4. check FRA balance
    // 5. construct tx
    // 6. check result
    Ok(())
}
