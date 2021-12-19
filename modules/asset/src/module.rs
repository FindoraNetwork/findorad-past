use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
    },
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};
use libfindora::asset::AssetType;

use crate::{utils, AssetInfo, Transaction};

#[abcf::module(name = "asset", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct AssetModule {
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub asset_infos: Map<AssetType, AssetInfo>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl AssetModule {}

/// Module's block logic.
#[abcf::application]
impl Application for AssetModule {
    type Transaction = Transaction;

    async fn check_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let tx = &req.tx;

        utils::check_define(&mut context.stateful.asset_infos, &tx.define_asset)?;
        utils::check_issue(&context.stateful.asset_infos, &tx.issue_asset)?;
        utils::check_transfer(&context.stateful.asset_infos, &tx.transfer_asset)?;

        Ok(Default::default())
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let tx = &req.tx;

        utils::check_define(&mut context.stateful.asset_infos, &tx.define_asset)?;
        utils::check_issue(&context.stateful.asset_infos, &tx.issue_asset)?;
        utils::check_transfer(&context.stateful.asset_infos, &tx.transfer_asset)?;

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl AssetModule {}
