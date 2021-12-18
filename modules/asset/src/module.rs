use abcf::{
    bs3::{merkle::append_only::AppendOnlyMerkle, model::{Value, Map}},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};
use libfindora::asset::{self, AssetType, AssetInfo};

#[abcf::module(name = "asset", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct AssetModule {
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub sf_value: Map<AssetType, AssetInfo>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl AssetModule {}

/// Module's block logic.
#[abcf::application]
impl Application for AssetModule {
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
impl AssetModule {}
