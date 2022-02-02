use abcf::{
    bs3::{merkle::append_only::AppendOnlyMerkle, model::Value, ValueStore},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCContext, RPCResponse, TxnContext,
};
use primitive_types::H160;

use crate::{
    rpc::{self, RuleVersionResponse},
    runtime::{self, version},
    transaction, Error, Result,
};

#[abcf::module(
    name = "rewards",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct RewardsModule {
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub rule: Value<Vec<u8>>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

fn load_version(store: &impl ValueStore<Vec<u8>>) -> Result<Option<H160>> {
    Ok(if let Some(code) = store.get()? {
        let v = runtime::version(&code)?;
        Some(v)
    } else {
        None
    })
}

#[abcf::rpcs]
impl RewardsModule {
    pub async fn rule_version<'a>(
        &mut self,
        ctx: &mut RPCContext<'a, Self>,
        _params: rpc::RuleVersionRequest,
    ) -> RPCResponse<rpc::RuleVersionResponse> {
        let rule = &ctx.stateful.rule;
        match load_version(rule) {
            Ok(e) => RPCResponse::new(RuleVersionResponse { version: e }),
            Err(e) => e.to_rpc_error().into(),
        }
    }
}

/// Module's block logic.
#[abcf::application]
impl Application for RewardsModule {
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
impl RewardsModule {}
