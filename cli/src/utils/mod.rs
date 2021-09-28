use libfindora::transaction::Transaction;
use ruc::*;
// use serde_json::{json, Value};

pub async fn send_tx(tx: &Transaction) -> Result<()> {
    let provider = abcf_sdk::providers::HttpGetProvider {};
    let r = abcf_sdk::sender::send_tx(provider, "broadcast_tx_sync", tx)
        .await
        .map_err(|e| eg!(format!("{:?}", e)))?;
    println!("resp: {:?}", r);

    Ok(())
}
