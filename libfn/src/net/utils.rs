use crate::{Error, Result};
use abcf_sdk::jsonrpc::endpoint::abci_query::Response;
use abcf_sdk::providers::Provider;
use serde::Deserialize;
use serde_json::Value;

pub async fn abci_query<T, P>(params: Value, provider: &mut P) -> Result<Option<T>>
where
    T: for<'de> Deserialize<'de>,
    P: Provider,
{
    let result = provider
        .request::<Value, Response>("abci_query", &params)
        .await
        .map_err(|e| Error::AbcfSdkError(format!("{:?}", e)))?;

    if let Some(val) = result {
        log::info!("response is: {:?}", val);
        let base64_str = base64::encode(&val.response.value);
        let bytes = base64::decode(&base64_str)?;
        let t = serde_json::from_slice::<T>(&bytes)?;
        Ok(Some(t))
    } else {
        Ok(None)
    }
}
