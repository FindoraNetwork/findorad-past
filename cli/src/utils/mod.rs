pub mod obj;

use std::collections::BTreeMap;

use abcf_sdk::providers::HttpGetProvider;
use fm_utxo::utxo_module_rpc::get_owned_outputs;
use libfindora::{transaction::Transaction, utxo::GetOwnedUtxoReq};
use ruc::*;
use zei::serialization::ZeiFromToBytes;
use zei::xfr::{asset_record::open_blind_asset_record, sig::XfrKeyPair, structs::AssetType};
use zei::xfr::sig::XfrSecretKey;

use crate::{config::Config};
use libfn::{AccountEntry, Entry};
use crate::utils::obj::Resp;

pub async fn send_tx(tx: &Transaction) -> Result<String> {
    let provider = abcf_sdk::providers::HttpGetProvider {};
    let r = abcf_sdk::sender::send_tx(provider, "broadcast_tx_sync", tx)
        .await
        .map_err(|e| eg!(format!("{:?}", e)))?;

    log::debug!("resp:{:?}",r);

    if r.is_none() {
        return Err(Box::from(d!("send tx return none".to_string())));
    }

    let r = r.unwrap();

    let resp = serde_json::from_value::<Resp>(r).c(d!())?;

    println!("{:#?}", resp);

    if resp.code != 0 {
        return Err(Box::from(d!(resp.log)));
    }

    Ok(resp.hash)
}

// pub async fn query_tx(hash: &str) -> Result<()> {
//     let provider = abcf_sdk::providers::HttpGetProvider {};
//
//     let r = abcf_sdk::sender::query_tx(provider, "tx", hash)
//         .await
//         .map_err(|e| eg!(format!("{:?}", e)))?;
//
//     log::debug!("resp:{:?}",r);
//
//     if r.is_none() {
//         return Err(Box::from(d!("send tx return none".to_string())));
//     }
//
//     let r = r.unwrap();
//
//     let mut resp = serde_json::from_value::<QueryResp>(r).c(d!())?;
//     resp.parse_tx()?;
//
//     if resp.tx_result.code != 0 {
//         return Err(Box::from(d!(resp.tx_result.log)));
//     }
//
//     println!("{:#?}",resp);
//
//     Ok(())
// }

pub async fn get_value_map(wallets: Vec<XfrKeyPair>) -> Result<BTreeMap<AssetType, u64>> {
    let params = GetOwnedUtxoReq {
        owners: wallets.iter().map(|kp| kp.get_pk()).collect(),
    };

    let provider = HttpGetProvider {};

    let result = get_owned_outputs(provider, params)
        .await
        .map_err(|e| eg!(format!("{:?}", e)))?;

    let mut value_map = BTreeMap::new();

    for oai in result.data.c(d!())?.outputs {
        let keypair = &wallets[oai.0];
        let output = oai.1.output;

        log::debug!("{:?}", output);

        let open_asset_record = open_blind_asset_record(&output.core, &output.owner_memo, keypair)?;

        log::debug!("Open Asset Recore is:{:?}", open_asset_record);

        let amount = open_asset_record.amount;
        let asset_type = open_asset_record.asset_type;

        if let Some(am) = value_map.get_mut(&asset_type) {
            *am += amount
        } else {
            value_map.insert(asset_type, amount);
        }
    }
    Ok(value_map)
}

pub async fn read_list(config: &Config, batch: &str) -> Result<Vec<Entry>> {
    let p = if batch == "" { "default" } else { batch };

    let path_dir = config.node.home.clone().join("batch");
    let path = path_dir.join(p);

    let list = if !path.exists() {
        tokio::fs::create_dir_all(path_dir).await.c(d!())?;
        Vec::new()
    } else {
        let content = tokio::fs::read_to_string(path).await.c(d!())?;
        serde_json::from_str(&content).c(d!())?
    };

    Ok(list)
}

pub async fn write_list(config: &Config, batch: &str, list: Vec<Entry>) -> Result<()> {
    let p = if batch == "" { "default" } else { batch };

    let path = config.node.home.clone().join("batch").join(p);

    let mut l = read_list(config, batch).await?;
    let mut list = list;
    l.append(&mut list);

    let content = serde_json::to_string_pretty(&l).c(d!())?;

    tokio::fs::write(path, content).await.c(d!())?;
    Ok(())
}

pub async fn clean_list(config: &Config, batch: &str) -> Result<()> {
    let p = if batch == "" { "default" } else { batch };

    let path = config.node.home.clone().join("batch").join(p);

    tokio::fs::remove_file(path).await.c(d!())?;

    Ok(())
}

pub async fn read_account_list(config: &Config) -> Result<Vec<AccountEntry>> {
    let path = config.node.home.clone().join("account");

    let list = if !path.exists() {
        tokio::fs::File::create(path).await.c(d!())?;
        Vec::new()
    } else {
        let content = tokio::fs::read_to_string(path).await.c(d!())?;
        serde_json::from_str(&content).c(d!())?
    };

    Ok(list)
}

pub async fn write_account_list(config: &Config, list: Vec<AccountEntry>, is_cover: bool) -> Result<()>{
    let path = config.node.home.clone().join("account");

    let l = if is_cover {
        list
    } else {
        let mut l = read_account_list(config).await?;
        let mut list = list;
        l.append(&mut list);
        l
    };

    let content = serde_json::to_string_pretty(&l).c(d!())?;

    tokio::fs::write(path, content).await.c(d!())?;
    Ok(())
}

pub async fn delete_account_one(config: &Config, index:usize) -> Result<AccountEntry> {
    let mut v = read_account_list(config).await?;
    let entry = v.remove(index);

    write_account_list(config, v, true).await.c(d!())?;
    Ok(entry)
}

pub fn account_to_keypair(entry:&AccountEntry) -> Result<XfrKeyPair>{
    let sk_bytes = base64::decode(&entry.base64.key_pair.secret_key).c(d!())?;
    let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;

    let keypair = sk.into_keypair();
    Ok(keypair)
}
