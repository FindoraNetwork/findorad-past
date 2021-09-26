use ruc::*;
use zei::{
    setup::PublicParams,
    xfr::{
        asset_record::AssetRecordType,
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
use hex_literal::hex;
use sha3::{Digest, Sha3_256};
use std::fs;
use serde_json::json;

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

pub fn issue_asset(
    secret_key: String,
    amount: u64,
    asset_type: u8,
    batch_file: Option<String>,
) -> Result<()>{

    let kp = secret_key_to_keypair(secret_key)?;

    let mut outputs = Vec::new();
    let mut tx = Transaction::default();
    let bar = BlindAssetRecord{
        amount: XfrAmount::NonConfidential(amount),
        asset_type: XfrAssetType::NonConfidential(AssetType::from_identical_byte(asset_type)),
        public_key: kp.pub_key,
    };
    let output = Output{ core: bar, operation: OutputOperation::IssueAsset};
    outputs.push(output);

    tx.outputs = outputs;

    let tx_msg = serde_json::to_vec(&tx).c(d!())?;
    let mut hasher = Sha3_256::new();
    hasher.update(tx_msg);

    let txid = hasher.finalize();
    tx.txid = txid.as_slice().to_vec();

    if is_batch(batch_file,&tx)? {
        return Ok(());
    }

    send_tx(&tx);
    Ok(())
}

pub fn transfer(
    from: String,
    to: String,
    amount: u64,
    asset_type: u8,
    batch_file: Option<String>,
) -> Result<()>{

    let from_kp = secret_key_to_keypair(from)?;
    let to_pk = public_key_from_base64(to)?;

    let provider = abcf_sdk::providers::HttpProvider{};
    let param = GetOwnedUtxoReq{ owner: from_kp.pub_key.zei_to_bytes() };
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
    let mut tx = Transaction::default();
    let mut inputs = vec![];
    let mut outputs = vec![];
    let mut sum = 0;

    for oop in gour.outputs {
        if sum >= amount {
            break;
        }
        sum += oop.core.amount.get_amount().unwrap();

        let mut input = Input{
            txid: oop.txid,
            n: oop.n,
            operation: InputOperation::TransferAsset,
        };

        // TODO : Calculated signatures eg. tx.signatures.push(sig)
        inputs.push(input);
    }

    if sum < amount {
        return Err(Box::from(d!("Insufficient balance")))
    }

    let bar_to_target = BlindAssetRecord{
        amount: XfrAmount::NonConfidential(amount),
        asset_type: XfrAssetType::NonConfidential(AssetType::from_identical_byte(asset_type)),
        public_key: to_pk
    };
    let output_to_target = Output{ core: bar_to_target, operation: OutputOperation::TransferAsset };
    outputs.push(output_to_target);

    // create balance outputs
    if sum > amount {
        let bar_to_balance = BlindAssetRecord{
            amount: XfrAmount::NonConfidential(sum - amount),
            asset_type: XfrAssetType::NonConfidential(AssetType::from_identical_byte(asset_type)),
            public_key: from_kp.pub_key
        };
        let output_to_balance = Output{ core: bar_to_balance, operation: OutputOperation::TransferAsset };
        outputs.push(output_to_balance);
    }


    tx.inputs = inputs;
    tx.outputs = outputs;

    let tx_msg = serde_json::to_vec(&tx).c(d!())?;
    let mut hasher = Sha3_256::new();
    hasher.update(tx_msg);

    let txid = hasher.finalize();
    tx.txid = txid.as_slice().to_vec();

    if is_batch(batch_file,&tx)? {
        return Ok(());
    }

    send_tx(&tx);
    Ok(())
}

fn is_batch(batch_file:Option<String>,tx:&Transaction) -> Result<bool> {
    if let Some(file) = batch_file {
        let data_bytes = fs::read(&file).c(d!())?;
        let mut data_vec = serde_json::from_slice::<Vec<Value>>(&*data_bytes).c(d!())?;
        let tx_json = serde_json::to_value(tx).c(d!())?;
        data_vec.push(tx_json);
        let data_bytes = serde_json::to_vec(&data_vec).c(d!())?;
        fs::write(&file, data_bytes).c(d!())?;

        return Ok(true);
    }

    return Ok(false);
}

pub fn send_tx(tx: &Transaction) -> Result<()>{
    let url = "http://127.0.0.1:26657/broadcast_tx_async";
    println!("tx:{:?}",tx);

    let tx_capnp = tx.to_bytes()?;
    // let tx_json = json!({
    //     "tx":tx_capnp
    // });

    // let tx_json_vec = serde_json::to_vec(&tx_json).c(d!())?;

    let tx_hex = hex::encode(&tx_capnp);
    println!("tx hex:{:?}",tx_hex);

    let tx_base64 = base64::encode_config(&tx_hex, base64::URL_SAFE);
    println!("tx base64:{:?}",tx_base64);

    let json_rpc = json!({
        "jsonrpc":"2.0",
        "id":"anything",
        "method":"broadcast_tx_sync",
        "params":{
            "tx":tx_hex
        }
    });

    println!("jsonrpc:{}",json_rpc);

    // let r = attohttpc::post(url)
    //     .header(attohttpc::header::CONTENT_TYPE, "application/json")
    //     .text(json_rpc)
    //     .send()
    //     .c(d!())?;

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        // let mut r = reqwest::Client::new()
        //     .post(url)
        //     .body(json_rpc.to_string())
        //     .send()
        //     .await
        //     .unwrap()
        //     .json::<Value>()
        //     .await
        //     .unwrap();
        //
        // println!("resp: {:?}",r);

        let mut r = reqwest::Client::new()
            .get(url)
            .query(&[("tx","0x".to_string() + tx_hex.as_str())])
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
            .unwrap();

        println!("resp: {:?}",r);
    });

    Ok(())
}

/// 1.issue
/// 2.utxo_module_rpc::get_owned_outputs
#[test]
fn test_issue_asset() -> Result<()>{

    let mut from = fs::read_to_string("/root/serkey6").c(d!())?;
    from = from.replace("\n", "");
    let asset_type = 0_u8;
    let amount = 100_u64;

    issue_asset(from.clone(),amount,asset_type,None).c(d!())?;

    let from_kp = secret_key_to_keypair(from)?;
    let provider = abcf_sdk::providers::HttpProvider{};
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

    let mut from = fs::read_to_string("/root/serkey6").c(d!())?;
    from = from.replace("\n", "");
    let asset_type = 0_u8;
    let amount = 10_u64;
    let to = "sJbhfszBQhctJTIqdfXr-6LifRdtiQHPLC5PRvouhUQ=";

    transfer(from.clone(), to.to_string(), amount, asset_type, None).c(d!())?;

    let from_kp = secret_key_to_keypair(from)?;
    let provider = abcf_sdk::providers::HttpProvider{};
    let param = GetOwnedUtxoReq{ owner: from_kp.pub_key.zei_to_bytes() };
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let result = utxo_module_rpc::get_owned_outputs(param, provider).await;
        println!("{:#?}",result);
    });

    Ok(())
}