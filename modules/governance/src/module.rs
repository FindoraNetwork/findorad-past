use abcf::{
    bs3::{merkle::empty::EmptyMerkle, model::Value},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};

use crate::transaction;

#[abcf::module(
    name = "governance",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct GovernanceModule {
    #[stateful(merkle = "EmptyMerkle")]
    pub sf_value: Value<u32>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl GovernanceModule {}

/// Module's block logic.
#[abcf::application]
impl Application for GovernanceModule {
    type Transaction = transaction::Transaction;

    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Execute transaction on state.
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
impl GovernanceModule {}
