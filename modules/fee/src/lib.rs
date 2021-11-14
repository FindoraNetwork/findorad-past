#![feature(generic_associated_types)]

use abcf::{
    bs3::model::Value,
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, StatefulBatch, StatelessBatch,
};
use libfindora::fee::{constant::FRA_FEE_AMOUNT, FeeTransaction};

#[abcf::module(name = "fee", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct FeeModule {
    #[stateful]
    pub sf_value: Value<u32>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl FeeModule {}

/// Module's block logic.
#[abcf::application]
impl Application for FeeModule {
    type Transaction = FeeTransaction;

    async fn check_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let tx = &req.tx;

        if tx.amount == FRA_FEE_AMOUNT {
            Ok(Default::default())
        } else {
            Err(abcf::Error::ABCIApplicationError(
                90001,
                String::from("Fee Error"),
            ))
        }
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let tx = &req.tx;

        if tx.amount == FRA_FEE_AMOUNT {
            Ok(Default::default())
        } else {
            Err(abcf::Error::ABCIApplicationError(
                90001,
                String::from("Fee Error"),
            ))
        }
    }
}

/// Module's methods.
#[abcf::methods]
impl FeeModule {}
