use crate::governance::{penalty_amount_power, ByzantineKind};
use crate::{validator_keys::ValidatorPublicKey, voting};
use abcf::module::types::RequestBeginBlock;
use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
    },
    manager::{AContext, TContext},
    module::types::{
        RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx, ResponseDeliverTx,
        ResponseEndBlock,
    },
    tm_protos::abci::ValidatorUpdate,
    Application, Stateful, StatefulBatch, Stateless, StatelessBatch,
};
use libfindora::staking::TendermintAddress;
use libfindora::staking::{
    self,
    voting::{Amount, Power},
};
use std::{collections::BTreeMap, mem};
use zei::xfr::sig::XfrPublicKey;

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

    #[stateful(merkle = "AppendOnlyMerkle")]
    pub validator_staker: Map<TendermintAddress, XfrPublicKey>,

    /// Global delegation amount.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub global_power: Value<Power>,

    /// Delegation amount by wallet address.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub delegation_amount: Map<XfrPublicKey, Amount>,

    /// Who delegate to which validator.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub delegators: Map<TendermintAddress, BTreeMap<XfrPublicKey, Amount>>,

    /// TendermintAddress to validatorPublicKey
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub validator_addr_pubkey: Map<TendermintAddress, ValidatorPublicKey>,

    /// Validator power.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub powers: Map<TendermintAddress, Power>,
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

    async fn begin_block(
        &mut self,
        _context: &mut AContext<Stateless<Self>, Stateful<Self>>,
        _req: &RequestBeginBlock,
    ) {
        let mut penalty_list = vec![];

        // get the list of validators to be punished
        for eve in _req.byzantine_validators.iter() {
            if let Some(validator) = &eve.validator {
                let bk = ByzantineKind::from_evidence_type(eve.r#type);
                if bk.is_err() {
                    log::debug!(
                        "height: {}, type: {}, msg: {}",
                        eve.height,
                        eve.r#type,
                        bk.unwrap_err()
                    );
                    return;
                }
                let bk = bk.unwrap();

                penalty_list.push((validator.clone(), bk));
            }
        }

        // get list of validators offline
        if let Some(lci) = &_req.last_commit_info {
            for vote in lci.votes.iter() {
                if let Some(validator) = &vote.validator {
                    // signed_last_block == false means validator is OffLine
                    if !vote.signed_last_block {
                        let bk = ByzantineKind::OffLine;
                        penalty_list.push((validator.clone(), bk));
                    }
                }
            }
        }

        let _ = penalty_amount_power(
            &mut _context.stateful.powers,
            &mut _context.stateful.global_power,
            &mut _context.stateful.delegation_amount,
            &mut _context.stateful.delegators,
            &mut _context.stateful.validator_staker,
            &penalty_list,
        );
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
                &mut context.stateful.validator_staker,
                &mut context.stateful.validator_addr_pubkey,
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
