use ethereum_types::{H160, H256, H64, U256, U64};
use jsonrpc_core::{BoxFuture, Result};
use web3_rpc_core::{
    types::{
        BlockNumber, Bytes, CallRequest, Filter, Index, Log, Receipt, RichBlock, SyncStatus,
        Transaction, TransactionRequest, Work,
    },
    EthApi,
};

use crate::error;

pub struct EthApiImpl {
    pub upstream: String,
}

impl EthApi for EthApiImpl {
    fn protocol_version(&self) -> BoxFuture<Result<u64>> {
        let upstream = self.upstream.clone();

        Box::pin(async move {
            apis::protocol_version(&upstream).await
        })
    }

    fn chain_id(&self) -> BoxFuture<Result<Option<U64>>> {
        // form td rpc
        Box::pin(async { Ok(Some(U64::zero())) })
    }

    fn accounts(&self) -> Result<Vec<H160>> {
        Ok(vec![])
    }

    fn balance(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Result<U256>> {
        Box::pin(async { Ok(U256::zero()) })
    }

    fn send_transaction(&self, _: TransactionRequest) -> BoxFuture<Result<H256>> {
        Box::pin(async { Ok(H256::default()) })
    }

    fn call(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<Result<Bytes>> {
        Box::pin(async { Ok(Bytes::default()) })
    }

    fn syncing(&self) -> BoxFuture<Result<SyncStatus>> {
        let upstream = self.upstream.clone();

        Box::pin(async move {
            apis::syncing(&upstream).await
        })
    }

    fn author(&self) -> BoxFuture<Result<H160>> {
        let upstream = self.upstream.clone();

        Box::pin(async move {
            apis::coinbase(&upstream).await
        })
    }

    fn is_mining(&self) -> BoxFuture<Result<bool>> {
        let upstream = self.upstream.clone();

        Box::pin(async move {
            apis::is_mining(&upstream).await
        })
    }

    fn gas_price(&self) -> BoxFuture<Result<U256>> {
        Box::pin(async { Ok(U256::zero()) })
    }

    fn block_number(&self) -> BoxFuture<Result<U256>> {
        let upstream = self.upstream.clone();

        Box::pin(async move {
            apis::block_number(&upstream).await
        })
    }

    fn storage_at(&self, _: H160, _: U256, _: Option<BlockNumber>) -> BoxFuture<Result<H256>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn block_by_hash(&self, _: H256, _: bool) -> BoxFuture<Result<Option<RichBlock>>> {
        Box::pin(async { Ok(None) })
    }

    fn block_by_number(&self, _: BlockNumber, _: bool) -> BoxFuture<Result<Option<RichBlock>>> {
        Box::pin(async { Ok(None) })
    }

    fn transaction_count(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Result<U256>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn block_transaction_count_by_hash(&self, _: H256) -> BoxFuture<Result<Option<U256>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn block_transaction_count_by_number(&self, _: BlockNumber) -> BoxFuture<Result<Option<U256>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn code_at(&self, _: H160, _: Option<BlockNumber>) -> BoxFuture<Result<Bytes>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn send_raw_transaction(&self, _: Bytes) -> BoxFuture<Result<H256>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn estimate_gas(&self, _: CallRequest, _: Option<BlockNumber>) -> BoxFuture<Result<U256>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn transaction_by_hash(&self, _: H256) -> BoxFuture<Result<Option<Transaction>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn transaction_by_block_hash_and_index(
        &self,
        _: H256,
        _: Index,
    ) -> BoxFuture<Result<Option<Transaction>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn transaction_by_block_number_and_index(
        &self,
        _: BlockNumber,
        _: Index,
    ) -> BoxFuture<Result<Option<Transaction>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn transaction_receipt(&self, _: H256) -> BoxFuture<Result<Option<Receipt>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    fn logs(&self, _: Filter) -> BoxFuture<Result<Vec<Log>>> {
        Box::pin(async { Ok(Default::default()) })
    }

    // ----------- Not impl.
    fn work(&self) -> Result<Work> {
        Err(error::no_impl())
    }

    fn submit_work(&self, _: H64, _: H256, _: H256) -> Result<bool> {
        Err(error::no_impl())
    }

    fn submit_hashrate(&self, _: U256, _: H256) -> Result<bool> {
        Err(error::no_impl())
    }

    fn hashrate(&self) -> Result<U256> {
        Err(error::no_impl())
    }
    fn uncle_by_block_hash_and_index(&self, _: H256, _: Index) -> Result<Option<RichBlock>> {
        Err(error::no_impl())
    }

    fn uncle_by_block_number_and_index(
        &self,
        _: BlockNumber,
        _: Index,
    ) -> Result<Option<RichBlock>> {
        Err(error::no_impl())
    }

    fn block_uncles_count_by_hash(&self, _: H256) -> Result<U256> {
        Err(error::no_impl())
    }

    fn block_uncles_count_by_number(&self, _: BlockNumber) -> Result<U256> {
        Err(error::no_impl())
    }
}

mod apis {
    use ethereum_types::{U256, H160};
    use jsonrpc_core::Result;
    use web3_rpc_core::types::{SyncStatus, SyncInfo};

    use crate::utils;

    pub async fn protocol_version(upstream: &str) -> Result<u64> {
        let result = utils::status(upstream).await?;
        Ok(result.node_info.protocol_version.app)
    }

    pub async fn syncing(upstream: &str) -> Result<SyncStatus> {
        let result = utils::status(upstream).await?;
        Ok(match result.sync_info.catching_up {
            true => SyncStatus::None,
            false => SyncStatus::Info(SyncInfo {
                starting_block: U256::from(result.sync_info.earliest_block_height),
                current_block: U256::from(result.sync_info.latest_block_height),
                highest_block: U256::from(result.sync_info.latest_block_height),
                warp_chunks_amount: None,
                warp_chunks_processed: None,
            })
        })
    }

    pub async fn coinbase(upstream: &str) -> Result<H160> {
        let result = utils::status(upstream).await?;
        Ok(H160::from_slice(&result.validator_info.address))
    }

    pub async fn is_mining(upstream: &str) -> Result<bool> {
        let result = utils::status(upstream).await?;
        Ok(result.validator_info.voting_power != 0)
    }

    pub async fn block_number(upstream: &str) -> Result<U256> {
        let result = utils::status(upstream).await?;
        Ok(U256::from(result.sync_info.latest_block_height))
    }
}

