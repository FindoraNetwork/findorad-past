use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
        ValueStore,
    },
    module::types::{RequestDeliverTx, ResponseDeliverTx, ResponseEndBlock},
    Application, TxnContext,
};
use fm_utxo::UtxoModule;
use libfindora::utxo::Output;

use crate::Transaction;

#[abcf::module(
    name = "coinbase",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
#[dependence(utxo = "UtxoModule")]
pub struct CoinbaseModule {
    pub block_height: i64,

    #[stateful(merkle = "AppendOnlyMerkle")]
    pub begin_index: Value<i64>,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub pending_outputs: Map<i64, Output>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl CoinbaseModule {}

/// Module's block logic.
#[abcf::application]
impl Application for CoinbaseModule {
    type Transaction = Transaction;

    async fn begin_block(
        &mut self,
        _context: &mut abcf::AppContext<'_, Self>,
        req: &abcf::tm_protos::abci::RequestBeginBlock,
    ) {
        if let Some(header) = &req.header {
            self.block_height = header.height;
        } else {
            // TODO: consider panic node.
            panic!("Got none header, Please reset node.");
        }
    }

    async fn end_block(
        &mut self,
        context: &mut abcf::AppContext<'_, Self>,
        _req: &abcf::tm_protos::abci::RequestEndBlock,
    ) -> ResponseEndBlock {
        if let Ok(_begin_index_data) = context.stateful.begin_index.get() {
            //             if let Some(begin_index) = begin_index_data {
            // // if begin_index <= begin_index
            //             }
        } else {
            // TODO: consider panic node.
            panic!("Read data from store failed.");
        }

        Default::default()
    }

    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        fm_utxo::utils::mint(
            &mut context.deps.utxo.stateful.outputs_set,
            &mut context.deps.utxo.stateless.owned_outputs,
            &req.tx.outputs,
        )?;

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl CoinbaseModule {}
