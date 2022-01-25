use abcf_sdk::{jsonrpc::endpoint, providers::{HttpGetProvider, Provider}};
use jsonrpc_core::Result;

use crate::error;

pub async fn net_info(upstream: &str) -> Result<endpoint::net_info::Response>
{
    let mut provider = HttpGetProvider {
        url: upstream.to_string(),
    };

    let result = provider
        .request::<(), endpoint::net_info::Response>("net_info", &())
        .await
        .map_err(error::sdk_error)?;
    log::debug!("abci query response:{:?}", result);

    if let Some(val) = result {
        Ok(val)
    } else {
        Err(error::empty_reponse())
    }
}

pub async fn status(upstream: &str) -> Result<endpoint::status::Response>
{
    let mut provider = HttpGetProvider {
        url: upstream.to_string(),
    };

    let result = provider
        .request::<(), endpoint::status::Response>("status", &())
        .await
        .map_err(error::sdk_error)?;
    log::debug!("abci query response:{:?}", result);

    if let Some(val) = result {
        Ok(val)
    } else {
        Err(error::empty_reponse())
    }
}
