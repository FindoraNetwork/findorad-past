use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{DoubleKeyMap, Map, Value},
    },
    module::types::{
        RequestBeginBlock, RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx,
    },
    AppContext, Application, RPCContext, RPCResponse, TxnContext,
};
use fm_chain::ChainModule;
use fm_utxo::UtxoModule;
use primitive_types::{H160, H256, U256};

use crate::{
    evm::{account::Account, vicinity::Vicinity},
    rpc::{self, EstimateGasResponse},
    transaction::EvmTransaction,
    utils, Transaction,
};

#[abcf::module(name = "evm", version = 1, impl_version = "0.1.1", target_height = 0)]
#[dependence(utxo = "UtxoModule", chain = "ChainModule")]
pub struct EvmModule {
    pub vicinity: Vicinity,

    #[stateful(merkle = "AppendOnlyMerkle")]
    pub accounts: Map<H160, Account>,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub storages: DoubleKeyMap<H160, H256, H256>,

    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl EvmModule {
    pub async fn metadata<'a>(
        &mut self,
        _ctx: &mut RPCContext<'a, Self>,
        _params: rpc::MetadataRequest,
    ) -> RPCResponse<rpc::MetadataResponse> {
        let metadata = rpc::MetadataResponse {
            chain_id: self.vicinity.chain_id.as_u64(),
            gas_price: self.vicinity.gas_price.as_u64(),
        };

        RPCResponse::new(metadata)
    }

    pub async fn estimate_gas<'a>(
        &mut self,
        _ctx: &mut RPCContext<'a, Self>,
        _params: EvmTransaction,
    ) -> RPCResponse<EstimateGasResponse> {
        let result = utils::estimate_gas();

        match result.0 {
            evm::ExitReason::Succeed(_) => RPCResponse::new(EstimateGasResponse { gas: result.1 }),
            _ => RPCResponse {
                code: 80001,
                data: Some(EstimateGasResponse { gas: result.1 }),
                message: format!("estimate gas error: {:?}", result.0),
            },
        }
    }
}

/// Module's block logic.
#[abcf::application]
impl Application for EvmModule {
    type Transaction = Transaction;

    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        // let tx = &req.tx;

        Ok(Default::default())
    }

    async fn begin_block(&mut self, _context: &mut AppContext<'_, Self>, req: &RequestBeginBlock) {
        let header = req.header.clone().expect("no header from tendermint");

        self.vicinity.block_hash = H256::from_slice(&req.hash);
        self.vicinity.block_number = U256::from(header.height);
        self.vicinity.block_coinbase = H160::from_slice(&header.proposer_address);
        self.vicinity.block_timestamp =
            U256::from(header.time.expect("no timestamp from tendermint").seconds);
    }

    async fn deliver_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl EvmModule {}
