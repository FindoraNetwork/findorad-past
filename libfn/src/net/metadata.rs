use abcf_sdk::providers::Provider;
use fm_evm::rpc::{MetadataRequest, MetadataResponse};

use crate::net::utils::abci_query;
use crate::{Error, Result};

pub async fn get<P: Provider>(provider: &mut P) -> Result<MetadataResponse> {
    let address_bytes = serde_json::to_vec(&MetadataRequest {})?;
    let hex_data = format!("0x{}", hex::encode(address_bytes));

    let hex_path = format!("0x{}", hex::encode("rpc/evm/metadata"));

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0i64,
        "data": hex_data,
    });

    if let Some(metadata) = abci_query::<MetadataResponse, P>(params, provider).await? {
        Ok(metadata)
    } else {
        Err(Error::NoResponse)
    }
}
