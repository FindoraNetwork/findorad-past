use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{DoubleKeyMap, Map, Value},
    },
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCResponse, TxnContext, RPCContext,
};
use fm_utxo::UtxoModule;
use primitive_types::{H160, H256, U256};

use crate::{evm::{account::Account, vicinity::Vicinity}, rpc, Transaction};

#[abcf::module(name = "evm", version = 1, impl_version = "0.1.1", target_height = 0)]
#[dependence(utxo = "UtxoModule")]
pub struct EvmModule {
    pub vicinity: Vicinity,

    #[stateful(merkle = "AppendOnlyMerkle")]
    pub accounts: Map<H160, Account>,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub storages: DoubleKeyMap<H160, U256, H256>,
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
