use abcf::{
    bs3::model::{Map, Value},
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, Event, Stateful, StatefulBatch, Stateless, StatelessBatch,
};
use libfindora::utxo::{OutputId, UtxoTransacrion};
use rand_chacha::ChaChaRng;
use serde::{Deserialize, Serialize};
use zei::{setup::PublicParams, xfr::structs::BlindAssetRecord};

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UtxoModule {
    params: PublicParams,
    prng: ChaChaRng,
    #[stateful]
    pub output_set: Map<OutputId, BlindAssetRecord>,
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl UtxoModule {}

/// Module's block logic.
#[abcf::application]
impl Application for UtxoModule {
    type Transaction = UtxoTransacrion;

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
    ) -> Result<ResponseDeliverTx> {
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl UtxoModule {}
