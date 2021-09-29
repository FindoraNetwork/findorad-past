use std::collections::BTreeMap;

use abcf_sdk::providers::HttpGetProvider;
use findorad_lib::utxo_module_rpc::get_owned_outputs;
use libfindora::{transaction::Transaction, utxo::GetOwnedUtxoReq};
use ruc::*;
use zei::xfr::{asset_record::open_blind_asset_record, sig::XfrKeyPair, structs::AssetType};

use crate::{config::Config, entry::Entry};

pub async fn send_tx(tx: &Transaction) -> Result<()> {
    let provider = abcf_sdk::providers::HttpGetProvider {};
    let r = abcf_sdk::sender::send_tx(provider, "broadcast_tx_sync", tx)
        .await
        .map_err(|e| eg!(format!("{:?}", e)))?;
    println!("resp: {:?}", r);

    Ok(())
}

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

