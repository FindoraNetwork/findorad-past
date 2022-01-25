use jsonrpc_core::{BoxFuture, Result};
use web3_rpc_core::{types::PeerCount, NetApi};

use crate::{utils::net_info, error};

pub struct NetApiImpl {
    pub upstream: String,
}

async fn peer_count(upstream: &str) -> Result<PeerCount> {
    let info = net_info(upstream).await?;
    let pc = PeerCount::U32(info.n_peers.try_into().map_err(error::convert_error)?);
    Ok(pc)
}

async fn is_listening(upstream: &str) -> Result<bool> {
    let info = net_info(upstream).await?;

    Ok(info.listening)
}

impl NetApi for NetApiImpl {
    fn version(&self) -> BoxFuture<Result<String>> {
        // get chain id.
        Box::pin(async { Ok(0.to_string()) })
    }

    fn peer_count(&self) -> BoxFuture<Result<PeerCount>> {
        let upstream = self.upstream.clone();

        Box::pin(async move { peer_count(&upstream).await })
    }

    fn is_listening(&self) -> BoxFuture<Result<bool>> {
        let upstream = self.upstream.clone();

        Box::pin(async move { is_listening(&upstream).await })
    }
}
