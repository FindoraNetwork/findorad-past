use abcf_sdk::{jsonrpc::endpoint::tx::ResultResponse as TxResp, providers::Provider};
use libfindora::Transaction;
use serde_json::Value;

use crate::{Error, Result};

pub async fn send_tx<P: Provider>(provider: &mut P, tx: Transaction) -> Result<TxResp> {
    let tx_bytes = tx.serialize()?;
    let hex_tx = hex::encode(tx_bytes);
    let tx_param = format!("0x{}", hex_tx);
    let params = serde_json::json!({
        "tx": tx_param,
    });

    println!("{:?}", params);

    let response = provider
        .request::<Value, TxResp>("broadcast_tx_sync", &params)
        .await
        .map_err(|e| Error::AbcfSdkError(format!("{:?}", e)))?;

    match response {
        Some(res) => {
            println!("{:?}", res);
            return Ok(res);
        }
        None => Err(Error::AbcfSdkError("response is empty".to_string())),
    }
}
