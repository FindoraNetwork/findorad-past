#![feature(generic_associated_types)]

use abcf::{
    bs3::model::Value,
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, StatefulBatch, StatelessBatch,
};
use libfindora::rewards;

#[abcf::module(
    name = "rewards",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct Module {
    #[stateful]
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
    type Transaction = rewards::Transaction;

    async fn check_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl Module {}
