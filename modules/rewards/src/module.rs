use std::collections::BTreeMap;

use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
        MapStore,
    },
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCContext, RPCResponse, TxnContext,
};
use libfindora::{asset::Amount, staking::TendermintAddress, Address};

use crate::{
    rpc::{self, RuleVersionResponse},
    runtime, transaction, Error,
};

#[abcf::module(
    name = "rewards",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct RewardsModule {
    pub rule: Option<Vec<u8>>,

    #[stateful(merkle = "AppendOnlyMerkle")]
    pub rewards: Map<TendermintAddress, BTreeMap<Address, Amount>>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

// fn load_version(store: &impl ValueStore<Vec<u8>>) -> Result<Option<H160>> {
// Ok(if let Some(code) = store.get()? {
//     let v = runtime::version(&code)?;
//     Some(v)
// } else {
//     None
// })
// }

#[abcf::rpcs]
impl RewardsModule {
    pub async fn rule_version<'a>(
        &mut self,
        _ctx: &mut RPCContext<'a, Self>,
        _params: rpc::RuleVersionRequest,
    ) -> RPCResponse<rpc::RuleVersionResponse> {
        if let Some(rule) = &self.rule {
            match runtime::version(&rule) {
                Ok(e) => RPCResponse::new(RuleVersionResponse { version: Some(e) }),
                Err(e) => e.to_rpc_error().into(),
            }
        } else {
            RPCResponse::new(RuleVersionResponse { version: None })
        }
    }
}

/// Module's block logic.
#[abcf::application]
impl Application for RewardsModule {
    type Transaction = transaction::Transaction;

    async fn check_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let res = ResponseCheckTx::default();

        for info in &req.tx.infos {
            if let Some(a) = context.stateful.rewards.get(&info.validator)? {
                if let Some(amount) = a.get(&info.delegator) {
                    if let None = amount.checked_sub(info.amount) {
                        return Err(Error::InsufficientBalance.to_application_error());
                    }
                } else {
                    return Err(Error::InsufficientBalance.to_application_error());
                }
            } else {
                return Err(Error::InsufficientBalance.to_application_error());
            }
        }

        Ok(res)
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let res = ResponseDeliverTx::default();

        for info in &req.tx.infos {
            if let Some(a) = context.stateful.rewards.get_mut(&info.validator)? {
                if let Some(amount) = a.get_mut(&info.delegator) {
                    *amount -= info.amount;
                    // Coinbase issue.
                } else {
                    return Err(Error::InsufficientBalance.to_application_error());
                }
            } else {
                return Err(Error::InsufficientBalance.to_application_error());
            }
        }

        Ok(res)
    }
}

/// Module's methods.
#[abcf::methods]
impl RewardsModule {}
