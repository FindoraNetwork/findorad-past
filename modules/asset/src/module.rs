use abcf::{
    bs3::{merkle::empty::EmptyMerkle, model::Value},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};
use libfindora::asset;

#[abcf::module(name = "asset", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct Module {
    #[stateful(merkle = "EmptyMerkle")]
    pub sf_value: Value<u32>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl Module {}

/// Module's block logic.
#[abcf::application]
impl Application for Module {
    type Transaction = asset::Transaction;

    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let tx = &req.tx;

        Ok(Default::default())
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let tx = &req.tx;

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl Module {}
