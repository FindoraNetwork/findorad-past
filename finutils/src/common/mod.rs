use ruc::*;
use zei::{
    setup::PublicParams,
    xfr::{
        asset_record::AssetRecordType,
        sig::{XfrKeyPair, XfrPublicKey, XfrSecretKey},
    },
};
use libfindora::utxo::{UtxoTransacrion, GetOwnedUtxoReq, GetOwnedUtxoResp};
use findorad_lib::utxo_module_rpc;
use zei::serialization::ZeiFromToBytes;
use tokio::runtime::Runtime;
use libfindora::transaction::{Transaction, Output, OutputOperation, Input, InputOperation};
use zei::xfr::structs::{BlindAssetRecord, XfrAmount, XfrAssetType, AssetType};
use serde_json::Value;
use hex_literal::hex;
use sha3::{Digest, Sha3_256};
use std::io::Read;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

pub fn restore_keypair_from_str_with_default(sk_str: Option<&str>) -> Result<XfrKeyPair> {
    if let Some(sk) = sk_str {
        let str = &format!("\"{}\"", sk);
        serde_json::from_str::<XfrSecretKey>(str)
            .map(|sk| sk.into_keypair())
            .c(d!("Invalid secret key"))
    } else {
        return Err(Box::from(d!("sk_str is none")))
    }
}

pub fn public_key_from_base64(pk: &str) -> Result<XfrPublicKey> {
    base64::decode_config(pk, base64::URL_SAFE)
        .c(d!())
        .and_then(|bytes| XfrPublicKey::zei_from_bytes(&bytes).c(d!()))
}

pub fn issue_asset(
    sk_str: &str,
    amount: u64,
    asset_type: u8
) -> Result<()>{

    let kp = restore_keypair_from_str_with_default(Some(sk_str))?;

    // let provider = abcf_sdk::providers::HttpProvider{};
    // let param = GetOwnedUtxoReq{ owner: kp.pub_key.zei_to_bytes() };
    // let rt = Runtime::new().unwrap();
    // let mut value = None;
    // rt.block_on(async {
    //     let result = utxo_module_rpc::get_owned_outputs(param, provider).await.unwrap();
    //     value = result;
    //     println!("{:?}",result);
    // });

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
    tx.txid = Some(txid.as_slice().to_vec());

    println!("{:#?}",tx);

    send_tx(&tx);
    Ok(())
}

pub fn transfer(
    from: &str,
    to: &str,
    amount: u64,
    asset_type: u8,
) -> Result<()>{
    let from_kp = restore_keypair_from_str_with_default(Some(from))?;
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
            signature: None
        };
        let msg = serde_json::to_vec(&input).c(d!())?;
        let signature = from_kp.sign(msg.as_slice());
        input.signature = Some(signature);

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

    let mut tx = Transaction::default();
    tx.inputs = inputs;
    tx.outputs = outputs;

    let tx_msg = serde_json::to_vec(&tx).c(d!())?;
    let mut hasher = Sha3_256::new();
    hasher.update(tx_msg);

    let txid = hasher.finalize();
    tx.txid = Some(txid.as_slice().to_vec());

    send_tx(&tx);
    Ok(())
}

pub fn send_tx(tx: &Transaction) -> Result<()>{
    let url = "http://127.0.0.1:26657";

    let tx_json = serde_json::to_string(&tx).c(d!())?;
    let tx_b64 = base64::encode_config(&tx_json.as_str(), base64::URL_SAFE);

    let json_rpc = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":\"anything\",\"method\":\"broadcast_tx_sync\",\"params\": {{\"tx\": \"{}\"}}}}",
        &tx_b64
    );

    let r = attohttpc::post(url)
        .header(attohttpc::header::CONTENT_TYPE, "application/json")
        .text(json_rpc)
        .send()
        .c(d!())?;
    println!("{:?}",r);
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

    issue_asset(&*from,amount,asset_type).c(d!())?;

    sleep(Duration::from_millis(30));

    let from_kp = restore_keypair_from_str_with_default(Some(&*from))?;
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

    Ok(())
}
