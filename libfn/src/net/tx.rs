use abcf_sdk::{
    jsonrpc::{endpoint::tx::ResultResponse as TxResp, Response},
    providers::Provider,
};
use serde_json::Value;

use crate::{Error, Result};

pub async fn _send_tx<P: Provider>(provider: &mut P, tx_bytes: Vec<u8>) -> Result<Option<TxResp>> {
    let hex_tx = hex::encode(tx_bytes);
    let tx_param = format!("0x{}", hex_tx);
    let params = serde_json::json!({
        "tx": tx_param,
    });

    let result = provider
        .request::<Value, Response<TxResp>>("broadcast_tx_sync", &params)
        .await
        .map_err(|e| Error::AbcfSdkError(format!("{:?}", e)))?;

    if let Some(resp) = result {
        Ok(resp.result)
    } else {
        Ok(None)
    }
}
