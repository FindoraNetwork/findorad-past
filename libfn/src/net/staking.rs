use crate::net::utils::abci_query;
use crate::{Error, Result};
use abcf_sdk::providers::Provider;
use libfindora::asset::Amount;
use libfindora::staking::{TendermintAddress, ValidatorPublicKey};
use libfindora::Address;
use std::collections::BTreeMap;

pub async fn get_validator_pubkey<P: Provider>(
    provider: &mut P,
    addr: TendermintAddress,
) -> Result<ValidatorPublicKey> {
    let addr_bytes = serde_json::to_vec(&addr)?;
    let hex_addr = hex::encode(addr_bytes);

    let path = format!("stateful/staking/validator_pubkey/0x{}", hex_addr);
    let hex_path = format!("0x{}", hex::encode(path));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
    });

    if let Some(v_pk) = abci_query::<ValidatorPublicKey, P>(params, provider).await? {
        Ok(v_pk)
    } else {
        Err(Error::NoResponse)
    }
}

pub async fn get_validator_staker<P: Provider>(
    provider: &mut P,
    addr: TendermintAddress,
) -> Result<Address> {
    let addr_bytes = serde_json::to_vec(&addr)?;
    let hex_addr = hex::encode(addr_bytes);

    let path = format!("stateful/staking/validator_staker/0x{}", hex_addr);
    let hex_path = format!("0x{}", hex::encode(path));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
    });

    if let Some(address) = abci_query::<Address, P>(params, provider).await? {
        Ok(address)
    } else {
        Err(Error::NoResponse)
    }
}

pub async fn get_global_power<P: Provider>(provider: &mut P) -> Result<u64> {
    let path = format!("stateful/staking/global_power/0x{}", hex::encode(""));
    let hex_path = format!("0x{}", hex::encode(path));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
    });

    if let Some(power) = abci_query::<u64, P>(params, provider).await? {
        Ok(power)
    } else {
        Err(Error::NoResponse)
    }
}

pub async fn get_delegation_amount<P: Provider>(provider: &mut P, addr: Address) -> Result<Amount> {
    let addr_bytes = serde_json::to_vec(&addr)?;
    let hex_addr = hex::encode(addr_bytes);

    let path = format!("stateless/staking/delegation_amount/0x{}", hex_addr);
    let hex_path = format!("0x{}", hex::encode(path));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
    });

    if let Some(a) = abci_query::<Amount, P>(params, provider).await? {
        Ok(a)
    } else {
        Err(Error::NoResponse)
    }
}

pub async fn get_delegators<P: Provider>(
    provider: &mut P,
    addr: TendermintAddress,
) -> Result<BTreeMap<Address, Amount>> {
    let addr_bytes = serde_json::to_vec(&addr)?;
    let hex_addr = hex::encode(addr_bytes);

    let path = format!("stateful/staking/delegators/0x{}", hex_addr);
    let hex_path = format!("0x{}", hex::encode(path));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
    });

    if let Some(map) = abci_query::<BTreeMap<Address, Amount>, P>(params, provider).await? {
        Ok(map)
    } else {
        Err(Error::NoResponse)
    }
}

pub async fn get_power<P: Provider>(provider: &mut P, addr: TendermintAddress) -> Result<u64> {
    let addr_bytes = serde_json::to_vec(&addr)?;
    let hex_addr = hex::encode(addr_bytes);

    let path = format!("stateful/staking/powers/0x{}", hex_addr);
    let hex_path = format!("0x{}", hex::encode(path));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
    });

    if let Some(power) = abci_query::<u64, P>(params, provider).await? {
        Ok(power)
    } else {
        Err(Error::NoResponse)
    }
}
