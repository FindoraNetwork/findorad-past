use jsonrpc_core::{BoxFuture, Result};
use web3_rpc_core::{types::PeerCount, NetApi};

use crate::utils::net_info;

pub struct NetApiImpl {
    pub upstream: String,
}

async fn peer_count(upstream: &str) -> Result<PeerCount> {
    let info = net_info(upstream).await?;
    let pc = PeerCount::U32(info.n_peers);
    Ok(pc)
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

    fn is_listening(&self) -> Result<bool> {
        Ok(true)
    }
}
