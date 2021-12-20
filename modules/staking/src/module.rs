use crate::{transaction::Operation, utils, Power, Result, Transaction, FRA_STAKING};
use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
    },
    module::types::RequestBeginBlock,
    module::types::{
        RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx, ResponseDeliverTx,
        ResponseEndBlock,
    },
    tm_protos::abci::ValidatorUpdate,
    Application, {AppContext, TxnContext},
};
use fm_coinbase::CoinbaseModule;
use libfindora::{
    asset::{Amount, FRA, XfrAmount},
    staking::{TendermintAddress, ValidatorPublicKey},
    utxo::Output,
    Address,
};
use std::{collections::BTreeMap, mem};

#[abcf::module(
    name = "staking",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
#[dependence(coinbase = "CoinbaseModule")]
pub struct StakingModule {
    /// Recording validator update info
    ///
    /// This field will send to tendermint when end block.
    pub vote_updaters: BTreeMap<ValidatorPublicKey, i64>,

    /// TendermintAddress to validatorPublicKey
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub validator_pubkey: Map<TendermintAddress, ValidatorPublicKey>,

    #[stateful(merkle = "AppendOnlyMerkle")]
    pub validator_staker: Map<TendermintAddress, Address>,

    /// Global delegation amount.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub global_power: Value<Power>,

    /// Delegation amount by wallet address.
    #[stateless]
    pub delegation_amount: Map<Address, Amount>,

    /// Who delegate to which validator.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub delegators: Map<TendermintAddress, BTreeMap<Address, Amount>>,

    /// Validator power.
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub powers: Map<TendermintAddress, Power>,
}

#[abcf::rpcs]
impl StakingModule {}

/// Module's block logic.
#[abcf::application]
impl Application for StakingModule {
    type Transaction = Transaction;

    async fn check_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let tx = &req.tx;

        self.apply_tx(context, tx)?;

        Ok(Default::default())
    }

    async fn begin_block(&mut self, _context: &mut AppContext<'_, Self>, _req: &RequestBeginBlock) {
    }

    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let tx = &req.tx;

        let mut res = self.apply_tx(context, tx)?;

        self.vote_updaters.append(&mut res);

        for info in &tx.infos {
            match info.operation {
                Operation::Undelegate(_) => {
                    let output = Output {
                        address: info.delegator.clone(),
                        amount: XfrAmount::NonConfidential(info.amount),
                        asset: FRA.asset_type,
                        owner_memo: None,
                    };
                    fm_coinbase::utils::mint(
                        context.deps.coinbase.module.block_height + FRA_STAKING.undelegate_block,
                        output,
                        &mut context.deps.coinbase.stateful.pending_outputs,
                    )?;
                }
                _ => {}
            }
        }

        Ok(Default::default())
    }

    async fn end_block(
        &mut self,
        _context: &mut AppContext<'_, Self>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock {
        let mut res = ResponseEndBlock::default();

        let vote_updaters = mem::take(&mut self.vote_updaters);

        let updates = vote_updaters
            .into_iter()
            .map(|(key, power)| ValidatorUpdate {
                pub_key: key.into(),
                power,
            })
            .collect();

        res.validator_updates = updates;

        res
    }
}

#[abcf::methods]
impl StakingModule {
    pub fn apply_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        tx: &Transaction,
    ) -> Result<BTreeMap<ValidatorPublicKey, i64>> {
        let mut res = BTreeMap::new();

        for info in &tx.infos {
            match &info.operation {
                Operation::Delegate(op) => {
                    let validator_pubkey = utils::apply_delegated(
                        &info.delegator,
                        op,
                        &mut context.stateful.validator_staker,
                        &mut context.stateful.validator_pubkey,
                    )?;

                    let power = utils::apply_global(
                        info.amount,
                        &op,
                        &mut context.stateful.global_power,
                        &mut context.stateful.powers,
                    )?;

                    let td_power: i64 = power.try_into()?;

                    res.insert(validator_pubkey, td_power);

                    utils::apply_detail(
                        &info.delegator,
                        info.amount,
                        &op,
                        &mut context.stateful.delegators,
                        &mut context.stateless.delegation_amount,
                    )?;
                }
                Operation::Undelegate(op) => utils::apply_undelegate_amount(
                    info.amount,
                    &info.delegator,
                    op,
                    &mut context.stateful.delegators,
                    &mut context.stateful.global_power,
                    &mut context.stateful.powers,
                    &mut context.stateless.delegation_amount,
                )?,
            }
        }

        Ok(res)
    }
}
