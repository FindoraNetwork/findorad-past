
use abcf::{
    bs3::model::{Map, Value},
    manager::{AContext, TContext},
    module::types::{
        RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx, ResponseDeliverTx,
        ResponseEndBlock,
    },
    tm_protos::abci::ValidatorUpdate,
    Application, Stateful, StatefulBatch, Stateless, StatelessBatch,
};
use libfindora::staking::{
    self,
    voting::{Amount, Power},
};
use zei::xfr::sig::XfrPublicKey;
use crate::{voting, validator_pubkey::ValidatorPublicKey};
use std::{mem, collections::BTreeMap};

#[abcf::module(
    name = "staking",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct StakingModule {
    /// Placeholder for abcf.
    #[stateless]
    pub sl_value: Value<u32>,

    /// Recording validator update info
    ///
    /// This field will send to tendermint when end block.
    pub vote_updaters: Vec<ValidatorUpdate>,

    /// Global delegation amount.
    #[stateful]
    pub global_power: Value<Power>,

    /// Delegation amount by wallet address.
    #[stateful]
    pub delegation_amount: Map<XfrPublicKey, Amount>,

    /// Who delegate to which validator.
    #[stateful]
    pub delegators: Map<ValidatorPublicKey, BTreeMap<XfrPublicKey, Amount>>,

    /// Validator power.
    #[stateful]
    pub powers: Map<ValidatorPublicKey, Power>,
}

#[abcf::rpcs]
impl StakingModule {}

/// Module's block logic.
#[abcf::application]
impl Application for StakingModule {
    type Transaction = staking::Transaction;

    async fn check_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    async fn deliver_tx(
        &mut self,
        context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let infos = &req.tx.infos;

        let mut updates = Vec::new();

        for info in infos {
            let mut update = voting::execute_staking(
                info,
                &mut context.stateful.global_power,
                &mut context.stateful.delegation_amount,
                &mut context.stateful.delegators,
                &mut context.stateful.powers,
            )?;
            updates.append(&mut update);
        }

        self.vote_updaters.append(&mut updates);

        Ok(Default::default())
    }

    async fn end_block(
        &mut self,
        _context: &mut AContext<Stateless<Self>, Stateful<Self>>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock {
        let mut res = ResponseEndBlock::default();

        res.validator_updates = mem::replace(&mut self.vote_updaters, Vec::new());

        res
    }
}

#[abcf::methods]
impl StakingModule {}
