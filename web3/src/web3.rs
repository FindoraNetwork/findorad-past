use ethereum_types::H256;
use jsonrpc_core::Result;
use sha3::{Digest, Keccak256};
use web3_rpc_core::{types::Bytes, Web3Api};

pub struct Web3ApiImpl {
    pub upstream: String,
}

impl Web3Api for Web3ApiImpl {
    fn client_version(&self) -> Result<String> {
        // version for td rpc app_version.
        Ok(String::new())
    }

    fn sha3(&self, input: Bytes) -> Result<H256> {
        Ok(H256::from_slice(
            Keccak256::digest(&input.into_vec()).as_slice(),
        ))
    }
}
