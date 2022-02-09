use abcf::{
    bs3::{merkle::empty::EmptyMerkle, model::Value},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};

use crate::{transaction::FRA_FEE_AMOUNT, Transaction, Error};

#[abcf::module(name = "fee", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct FeeModule {
    pub gas: u64,

    #[stateful(merkle = "EmptyMerkle")]
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
    type Transaction = Transaction;

    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let tx = &req.tx;

        if tx.amount >= FRA_FEE_AMOUNT {
            let mut resp = ResponseCheckTx::default();

            resp.gas_wanted = tx.amount.try_into().map_err(Error::TryFromIntError)?;
            let gas_used = FRA_FEE_AMOUNT + self.gas;
            resp.gas_used = gas_used.try_into().map_err(Error::TryFromIntError)?;

            self.gas = 0;

            Ok(resp)
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
        _context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let tx = &req.tx;

        if tx.amount >= FRA_FEE_AMOUNT {
            let mut resp = ResponseDeliverTx::default();

            resp.gas_wanted = tx.amount.try_into().map_err(Error::TryFromIntError)?;
            let gas_used = FRA_FEE_AMOUNT + self.gas;
            resp.gas_used = gas_used.try_into().map_err(Error::TryFromIntError)?;

            self.gas = 0;

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
