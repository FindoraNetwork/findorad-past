use abcf::{
    bs3::{merkle::append_only::AppendOnlyMerkle, model::Value, MapStore},
    module::types::{
        RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx, ResponseEndBlock,
    },
    Application, TxnContext,
};
use fm_utxo::UtxoModule;

use crate::Transaction;

#[abcf::module(name = "evm", version = 1, impl_version = "0.1.1", target_height = 0)]
#[dependence(utxo = "UtxoModule")]
pub struct EvmModule {
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub pending_outputs: Value<u32>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl EvmModule {}

/// Module's block logic.
#[abcf::application]
impl Application for EvmModule {
    type Transaction = Transaction;

    async fn check_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let tx = &req.tx;

        for account in &tx.txs {}

        Ok(Default::default())
    }

    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl EvmModule {}
