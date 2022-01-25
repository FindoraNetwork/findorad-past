use jsonrpc_core::{BoxFuture, Result};
use web3_rpc_core::{types::PeerCount, NetApi};

pub struct NetApiImpl {
    pub upstream: String,
}

impl NetApi for NetApiImpl {
    fn version(&self) -> BoxFuture<Result<String>> {
        // get chain id.
        Box::pin(async { Ok(0.to_string()) })
    }

    fn peer_count(&self) -> BoxFuture<Result<PeerCount>> {
        // let upstream = self
        Box::pin(async { Ok(PeerCount::U32(0)) })
    }

    fn is_listening(&self) -> Result<bool> {
        Ok(true)
    }
}
