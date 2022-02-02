use std::{collections::BTreeMap};

use abcf::{
    bs3::{merkle::append_only::AppendOnlyMerkle, model::{Value, Map}, ValueStore, MapStore},
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCContext, RPCResponse, TxnContext,
};
use libfindora::{Address, staking::TendermintAddress, asset::Amount};
use primitive_types::H160;

use crate::{
    rpc::{self, RuleVersionResponse},
    transaction, Error, Result, runtime,
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
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub rewards: Map<TendermintAddress, BTreeMap<Address, Amount>>,
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
        context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let res = ResponseCheckTx::default();

        for info in &req.tx.infos {
            if let Some(a) = context.stateful.rewards.get(&info.validator)? {
                if let Some(amount) = a.get(&info.delegator) {
                    if let None = amount.checked_sub(info.amount) {
                        return Err(Error::InsufficientBalance.to_application_error())
                    }
                } else {
                    return Err(Error::InsufficientBalance.to_application_error())
                }
            } else {
                return Err(Error::InsufficientBalance.to_application_error())
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
                    return Err(Error::InsufficientBalance.to_application_error())
                }
            } else {
                return Err(Error::InsufficientBalance.to_application_error())
            }
        }

        Ok(res)
    }
}

/// Module's methods.
#[abcf::methods]
impl RewardsModule {}
