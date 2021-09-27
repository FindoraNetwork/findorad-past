use ruc::*;
use zei::{
    xfr::{
        sig::{XfrKeyPair, XfrPublicKey, XfrSecretKey},
        structs::{BlindAssetRecord, XfrAmount, XfrAssetType, AssetType},
    },
    serialization::ZeiFromToBytes,
};
use libfindora::transaction::{Transaction, Output, OutputOperation, Input, InputOperation};
use findorad_lib::utxo_module_rpc;
use libfindora::utxo::{GetOwnedUtxoReq, GetOwnedUtxoResp};
use tokio::runtime::Runtime;
use serde_json::Value;
use sha3::{Digest, Sha3_256};
use std::path::PathBuf;
use crate::entry::{IssueBatchEntry, TransferBatchEntry, BalanceEntry};
use std::collections::HashMap;

pub fn secret_key_to_keypair(secret_key: String) -> Result<XfrKeyPair> {
    let str = &format!("\"{}\"", secret_key);
    serde_json::from_str::<XfrSecretKey>(str)
        .map(|sk| sk.into_keypair())
        .c(d!("Invalid secret key"))
}

pub fn public_key_from_base64(pk: String) -> Result<XfrPublicKey> {
    base64::decode_config(&pk, base64::URL_SAFE)
        .c(d!())
        .and_then(|bytes| XfrPublicKey::zei_from_bytes(&bytes).c(d!()))
}

pub fn issue_tx(batch_vec:Vec<IssueBatchEntry>) -> Result<Transaction> {
    let mut tx = Transaction::default();

    for entry in batch_vec.iter() {

        let bar = BlindAssetRecord{
            amount: XfrAmount::NonConfidential(entry.amount),
            asset_type: entry.asset_type.clone(),
            public_key: entry.keypair.pub_key,
        };

        let output = Output{ core: bar, operation: OutputOperation::IssueAsset};
        tx.outputs.push(output);

    }

    let tx_msg = serde_json::to_vec(&tx).c(d!())?;
    let mut hasher = Sha3_256::new();
    hasher.update(tx_msg);

    let txid = hasher.finalize();
    tx.txid = txid.as_slice().to_vec();

    Ok(tx)
}

pub fn transfer_tx (batch_map:HashMap<XfrPublicKey,Vec<TransferBatchEntry>>) -> Result<Transaction> {
    let mut tx = Transaction::default();

    for (pub_key,vec) in batch_map.iter() {
        let provider = abcf_sdk::providers::HttpPostProvider{};
        let param = GetOwnedUtxoReq{ owner: pub_key.zei_to_bytes() };
        let rt = Runtime::new().unwrap();
        let mut value:Option<Value> = None;
        rt.block_on(async {
            let result = utxo_module_rpc::get_owned_outputs(param, provider).await.unwrap();
            value = result;
        });

        if value.is_none() {
            return Err(Box::from(d!("Insufficient balance")))
        }

        let gour = serde_json::from_value::<GetOwnedUtxoResp>(value.unwrap()).c(d!())?;

        // ty ---> amount map
        let mut balance_total_map:HashMap<AssetType,BalanceEntry> = HashMap::new();

        vec.iter().for_each(|entry|{

            let ty = entry.asset_type.clone();

            if !ty.is_confidential() {
                // safe unsafe
                let at = ty.get_asset_type().unwrap();
                if let Some(be) = balance_total_map.get_mut(&at) {
                    be.amount += entry.amount;
                } else {
                    let be = BalanceEntry {
                        amount: entry.amount,
                        balance: 0,
                    };

                    balance_total_map.insert(at.clone(),be);
                }

                // transfer output
                let bar_to_target = BlindAssetRecord{
                    amount: XfrAmount::NonConfidential(entry.amount),
                    asset_type: entry.asset_type.clone(),
                    public_key: entry.to
                };

                let output_to_target = Output{ core: bar_to_target, operation: OutputOperation::TransferAsset };
                tx.outputs.push(output_to_target);
            }

        });

        for oop in gour.outputs {

            if !oop.core.asset_type.is_confidential() {
                let oop_am = oop.core.amount.get_amount().unwrap();
                // safe unwrap
                let at = oop.core.asset_type.get_asset_type().unwrap();
                if let Some(be) = balance_total_map.get_mut(&at) {
                    if be.amount == 0 {
                        continue;
                    }

                    if oop_am >= be.amount {
                        be.amount = 0;
                        be.balance = oop_am - be.amount;
                    } else {
                        be.amount = be.amount - oop_am;
                    }

                    let input = Input{
                        txid: oop.txid,
                        n: oop.n,
                        operation: InputOperation::TransferAsset,
                    };

                    tx.inputs.push(input)
                }
            }

        }

        // balance and determine
        for (ty,be) in balance_total_map.iter() {

            // Determine if there is enough money for the transfer
            if be.amount > 0 {
                return Err(Box::from(d!("Insufficient balance")));
            }

            // balance outputs
            let bar_to_balance = BlindAssetRecord{
                amount: XfrAmount::NonConfidential(be.balance),
                asset_type: XfrAssetType::NonConfidential(*ty),
                public_key: pub_key.clone()
            };
            let output_to_balance = Output{ core: bar_to_balance, operation: OutputOperation::TransferAsset };
            tx.outputs.push(output_to_balance);
        }
    }

    let tx_msg = serde_json::to_vec(&tx).c(d!())?;
    let mut hasher = Sha3_256::new();
    hasher.update(tx_msg);

    let txid = hasher.finalize();
    tx.txid = txid.as_slice().to_vec();

    Ok(tx)
}

pub fn save_issue_to_batch(path: &PathBuf, json: IssueBatchEntry) -> Result<()> {
    let mut data_vec = read_issue_from_batch(path).c(d!())?;
    data_vec.push(json);
    let data_bytes = serde_json::to_vec(&data_vec).c(d!())?;
    std::fs::write(path.as_path(),data_bytes).c(d!())?;
    Ok(())
}

pub fn read_issue_from_batch(path: &PathBuf) -> Result<Vec<IssueBatchEntry>> {
    let data = std::fs::read(path.as_path()).c(d!())?;
    let data_vec = serde_json::from_slice::<Vec<IssueBatchEntry>>(&*data).c(d!())?;
    Ok(data_vec)
}

pub fn save_transfer_to_batch(path: &PathBuf, json:TransferBatchEntry) -> Result<()>{
    let mut data_map = read_transfer_from_batch(path).c(d!())?;
    if let Some(pubkey_entry_vec) = data_map.get_mut(&json.from.pub_key) {
        pubkey_entry_vec.push(json);
    } else {
        let pub_key = json.from.pub_key.clone();
        let v = vec![json];
        data_map.insert(pub_key, v);
    }

    let data_bytes = serde_json::to_vec(&data_map).c(d!())?;
    std::fs::write(path.as_path(),data_bytes).c(d!())?;
    Ok(())
}

pub fn read_transfer_from_batch(path: &PathBuf) -> Result<HashMap<XfrPublicKey,Vec<TransferBatchEntry>>> {
    let data = std::fs::read(path.as_path()).c(d!())?;
    let data_map
        = serde_json::from_slice::<HashMap<XfrPublicKey,Vec<TransferBatchEntry>>>(&*data).c(d!())?;
    Ok(data_map)
}

pub fn send_tx(tx: &Transaction) -> Result<()>{

    let tx_capnp = tx.to_bytes()?;

    let tx_hex = hex::encode(&tx_capnp);
    println!("tx hex:{:?}",tx_hex);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let provider = abcf_sdk::providers::HttpGetProvider{};
        let r = utxo_module_rpc::send_tx(tx_hex,"broadcast_tx_async",provider).await;
        println!("resp: {:?}",r);
    });

    Ok(())
}

/// 1.issue
/// 2.utxo_module_rpc::get_owned_outputs
#[test]
fn test_issue_asset() -> Result<()>{

    let mut from = std::fs::read_to_string("/root/serkey6").c(d!())?;
    from = from.replace("\n", "");
    let asset_type = 0_u8;
    let amount = 100_u64;

    issue_asset(from.clone(),amount,asset_type,None).c(d!())?;

    let from_kp = secret_key_to_keypair(from)?;
    let provider = abcf_sdk::providers::HttpPostProvider{};
    let param = GetOwnedUtxoReq{ owner: from_kp.pub_key.zei_to_bytes() };
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result = utxo_module_rpc::get_owned_outputs(param, provider).await;
        println!("{:#?}",result);
    });

    Ok(())
}

#[test]
fn test_transfer() -> Result<()>{

    let mut from = std::fs::read_to_string("/root/serkey6").c(d!())?;
    from = from.replace("\n", "");
    let asset_type = 0_u8;
    let amount = 10_u64;
    let to = "sJbhfszBQhctJTIqdfXr-6LifRdtiQHPLC5PRvouhUQ=";

    transfer(from.clone(), to.to_string(), amount, asset_type, None).c(d!())?;

    let from_kp = secret_key_to_keypair(from)?;
    let provider = abcf_sdk::providers::HttpPostProvider{};
    let param = GetOwnedUtxoReq{ owner: from_kp.pub_key.zei_to_bytes() };
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result = utxo_module_rpc::get_owned_outputs(param, provider).await;
        println!("{:#?}",result);
    });

    Ok(())
}