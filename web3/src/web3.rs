use ethereum_types::H256;
use jsonrpc_core::{Result, BoxFuture};
use sha3::{Digest, Keccak256};
use web3_rpc_core::{types::Bytes, Web3Api};

use crate::utils;

pub struct Web3ApiImpl {
    pub upstream: String,
}

async fn client_version(upstream: &str) -> Result<String> {
    let result = utils::status(upstream).await?;
    Ok(result.node_info.protocol_version.app.to_string())
}

impl Web3Api for Web3ApiImpl {
    fn client_version(&self) -> BoxFuture<Result<String>> {
        let upstream = self.upstream.clone();

        Box::pin(async move {
            client_version(&upstream).await
        })
    }

    fn sha3(&self, input: Bytes) -> Result<H256> {
        Ok(H256::from_slice(
            Keccak256::digest(&input.into_vec()).as_slice(),
        ))
    }
}
