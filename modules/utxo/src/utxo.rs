use abcf::{
    bs3::{merkle::append_only::AppendOnlyMerkle, model::Map, MapStore},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};
use libfindora::{
    utxo::{Output, OutputId, UtxoTransaction},
    Address,
};
use rand_chacha::ChaChaRng;
use zei::setup::PublicParams;

use crate::utils;

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UtxoModule {
    params: PublicParams,
    prng: ChaChaRng,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub outputs_set: Map<OutputId, Output>,
    #[stateless]
    pub owned_outputs: Map<Address, Vec<OutputId>>,
}

#[abcf::rpcs]
impl UtxoModule {}

/// Module's block logic.
#[abcf::application]
impl Application for UtxoModule {
    type Transaction = UtxoTransaction;

    async fn check_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        utils::check_tx(
            &mut self.params,
            &mut self.prng,
            &context.stateful.outputs_set,
            &req.tx,
        )?;

        Ok(Default::default())
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let tx: &UtxoTransaction = &req.tx;

        let owned_outputs = utils::deliver_tx(
            &mut self.params,
            &mut self.prng,
            &mut context.stateful.outputs_set,
            &tx,
        )?;

        for (owner, ops) in owned_outputs.into_iter() {
            if let Some(v) = context.stateless.owned_outputs.get_mut(&owner)? {
                utils::insert_by_operation(v, ops)?;
            } else {
                let mut v = Vec::new();
                utils::insert_by_operation(&mut v, ops)?;

                context.stateless.owned_outputs.insert(owner.clone(), v)?;
            }
        }

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl UtxoModule {}
