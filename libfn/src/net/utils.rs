use serde_json::Value;
use serde::{Deserialize, Serialize};
use abcf_sdk::providers::Provider;
use abcf_sdk::jsonrpc::endpoint::abci_query::Response;
use crate::{Result, Error};

pub async fn abci_query<T,P>(params: Value, provider: &mut P,) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
        P: Provider,

{
    let result = provider.request::<Value, Response>("abci_query", &params)
        .await
        .map_err(|e|Error::AbcfSdkError(format!("{:?}",e)))?;

    if let Some(val) = result {
        let base64_str = base64::encode(&val.response.value);
        let bytes = base64::decode(&base64_str)?;
        let t = serde_json::from_slice::<T>(&bytes)?;
        Ok(t)
    } else {
        log::debug!("request abci_query response:{:?}", result);
        Err(Error::AbcfSdkError("request none".to_string()))
    }
}