#![feature(generic_associated_types)]

mod voting;

mod validator_pubkey;
pub use validator_pubkey::ValidatorPublicKey;

mod staking;
pub use staking::StakingModule;

mod delegate;

mod undelegate;

// use crate::voting::Voting;
// use abcf::bs3::model::Map;
// use abcf::bs3::MapStore;
// use abcf::bs3::ValueStore;
// use abcf::tm_protos::abci::ValidatorUpdate;
// use abcf::tm_protos::crypto;
// use abcf::Stateful;
// use abcf::Stateless;
// use abcf::{
//     bs3::model::Value,
//     manager::{AContext, TContext},
//     module::types::{
//         RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx, ResponseDeliverTx,
//         ResponseEndBlock,
//     },
//     Application, StatefulBatch, StatelessBatch,
// };
// use libfindora::staking::voting::Amount;
// use libfindora::staking::voting::MAX_DELEGATION_AMOUNT;
// use libfindora::staking::voting::MAX_POWER_PERCENT_PER_VALIDATOR;
// use libfindora::staking::voting::MAX_TOTAL_POWER;
// use libfindora::staking::voting::MIN_DELEGATION_AMOUNT;
// use libfindora::staking::voting::STAKING_VALIDATOR_MIN_POWER;
// // use libfindora::staking::{self, StakingInfo};
// use ruc::*;
// use std::ops::Deref;
// use zei::xfr::sig::XfrPublicKey;
//
// #[abcf::module(
//     name = "staking",
//     version = 1,
//     impl_version = "0.1.1",
//     target_height = 0
// )]
//
// pub struct StakingModule {
//     #[stateless]
//     pub sl_value: Value<u32>,
//
//     // Holding current staking transaction
//     pub infos: Vec<StakingInfo>,
//
//     #[stateful]
//     pub global_delegation_amount: Value<Amount>,
//
//     #[stateful]
//     pub delegation_amount_map: Map<XfrPublicKey, Amount>,
//
//     #[stateful]
//     pub delegator_validator_map: Map<XfrPublicKey, ValidatorPublicKey>,
//
//     #[stateful]
//     pub validator_power_map: Map<ValidatorPublicKey, Amount>,
// }
//
// #[abcf::rpcs]
// impl StakingModule {}
//
// /// Module's block logic.
// #[abcf::application]
// impl Application for StakingModule {
//     type Transaction = staking::Transaction;
//
//     async fn check_tx(
//         &mut self,
//         _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
//         _req: &RequestCheckTx<Self::Transaction>,
//     ) -> abcf::Result<ResponseCheckTx> {
//         Ok(Default::default())
//     }
//
//     /// Execute transaction on state.
//     async fn deliver_tx(
//         &mut self,
//         _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
//         req: &RequestDeliverTx<Self::Transaction>,
//     ) -> abcf::Result<ResponseDeliverTx> {
//         self.infos = req.tx.infos.clone();
//
//         Ok(Default::default())
//     }
//
//     //End Block.
//     async fn end_block(
//         &mut self,
//         context: &mut AContext<Stateless<Self>, Stateful<Self>>,
//         _req: &RequestEndBlock,
//     ) -> ResponseEndBlock {
//         let mut res = ResponseEndBlock::default();
//
//         let validator_updates = self.validator_updates(context);
//         res.validator_updates = validator_updates;
//
//         res
//     }
// }
//
// /// Module's methods.
// #[abcf::methods]
// impl StakingModule {
//     pub fn validator_updates(
//         &mut self,
//         context: &mut AContext<Stateless<Self>, Stateful<Self>>,
//     ) -> Vec<ValidatorUpdate> {
//         let mut validator_updates: Vec<ValidatorUpdate> = Vec::new();
//
//         for staking_info in &self.infos {
//             match &staking_info.operation {
//                 staking::Operation::Delegate(delegate) => {
//                     if StakingModule::check_voting_power(
//                         &staking_info.delegator,
//                         &delegate.validator,
//                         staking_info.amount,
//                         context,
//                     )
//                     .is_ok()
//                     {
//                         let key =
//                             ValidatorPublicKey::from_crypto_publickey(&delegate.validator).unwrap();
//                         // let cur_power = *self.validator_power_map.get(&key).unwrap();
//                         let cur_power = context
//                             .stateful
//                             .validator_power_map
//                             .get(&key)
//                             .unwrap()
//                             .unwrap();
//                         let pending_power = staking_info.amount as i64;
//                         let power = (*cur_power.deref() as i64).saturating_add(pending_power);
//
//                         // update global_delegation_amount
//                         // self.global_delegation_amount += pending_power as u64;
//                         if let Ok(Some(x)) = context.stateful.global_delegation_amount.get() {
//                             let newx = x.deref() + pending_power as u64;
//                             context.stateful.global_delegation_amount.set(newx).unwrap();
//                         }
//
//                         // update validator_power_map
//                         // self.validator_power_map.insert(key.clone(), power as u64);
//                         context
//                             .stateful
//                             .validator_power_map
//                             .insert(key.clone(), power as u64)
//                             .unwrap();
//
//                         // delegation_amount_map
//                         // *self
//                         //     .delegation_amount_map
//                         //     .entry(staking_info.delegator.clone())
//                         //     .or_insert(0) += pending_power as u64;
//                         if let Ok(Some(x)) = context
//                             .stateful
//                             .delegation_amount_map
//                             .get(&staking_info.delegator.clone())
//                         {
//                             let newx = x.deref() + pending_power as u64;
//                             context
//                                 .stateful
//                                 .delegation_amount_map
//                                 .insert(staking_info.delegator.clone(), newx)
//                                 .unwrap();
//                         } else {
//                             context
//                                 .stateful
//                                 .delegation_amount_map
//                                 .insert(staking_info.delegator.clone(), pending_power as u64)
//                                 .unwrap();
//                         }
//
//                         // delegator_validator_map
//                         // self.delegator_validator_map
//                         //     .insert(staking_info.delegator, key.clone());
//                         context
//                             .stateful
//                             .delegator_validator_map
//                             .insert(staking_info.delegator, key.clone())
//                             .unwrap();
//
//                         validator_updates.push(ValidatorUpdate {
//                             pub_key: Some(key.to_crypto_publickey().unwrap()),
//                             power,
//                         });
//                     }
//                 }
//                 staking::Operation::Undelegate(undelegate) => {
//                     if StakingModule::check_voting_power(
//                         &staking_info.delegator,
//                         &undelegate.validator,
//                         staking_info.amount,
//                         context,
//                     )
//                     .is_ok()
//                     {
//                         // let key = crypto_key_2_vec(&undelegate.validator);
//                         let key = ValidatorPublicKey::from_crypto_publickey(&undelegate.validator)
//                             .unwrap();
//                         let cur_power = context
//                             .stateful
//                             .validator_power_map
//                             .get(&key)
//                             .unwrap()
//                             .unwrap();
//                         let pending_power = staking_info.amount as i64;
//                         let power = (*cur_power.deref() as i64).saturating_sub(pending_power);
//
//                         // update global_delegation_amount
//                         // self.global_delegation_amount -= pending_power as u64;
//                         if let Ok(Some(x)) = context.stateful.global_delegation_amount.get() {
//                             let newx = x.deref() - pending_power as u64;
//                             context.stateful.global_delegation_amount.set(newx).unwrap();
//                         }
//
//                         // update validator_power_map
//                         // self.validator_power_map.insert(key.clone(), power as u64);
//                         context
//                             .stateful
//                             .validator_power_map
//                             .insert(key.clone(), power as u64)
//                             .unwrap();
//
//                         // delegation_amount_map
//                         // if let Some(bl) =
//                         //     self.delegation_amount_map.get_mut(&staking_info.delegator)
//                         // {
//                         //     *bl -= pending_power as u64;
//                         // }
//                         if let Ok(Some(x)) = context
//                             .stateful
//                             .delegation_amount_map
//                             .get(&staking_info.delegator.clone())
//                         {
//                             let newx = x.deref() - pending_power as u64;
//                             context
//                                 .stateful
//                                 .delegation_amount_map
//                                 .insert(staking_info.delegator.clone(), newx)
//                                 .unwrap();
//                         }
//
//                         // delegator_validator_map
//                         // self.delegator_validator_map
//                         //     .insert(staking_info.delegator, key.clone());
//                         context
//                             .stateful
//                             .delegator_validator_map
//                             .insert(staking_info.delegator, key.clone())
//                             .unwrap();
//
//                         validator_updates.push(ValidatorUpdate {
//                             pub_key: Some(key.to_crypto_publickey().unwrap()),
//                             power,
//                         });
//                     }
//                 }
//             }
//         }
//
//         validator_updates
//     }
//
//     fn check_voting_power(
//         _delegator: &XfrPublicKey,
//         validator: &crypto::PublicKey,
//         amount: Amount,
//         context: &mut AContext<Stateless<Self>, Stateful<Self>>,
//     ) -> Result<()> {
//         // Done self delegation
//         // let key = crypto_key_2_vec(&validator);
//         // let key = ValidatorPublicKey::from_crypto_publickey(&validator).unwrap();
//         // let is_self_delegation = self.validator_power_map.contains_key(&key);
//
//         // if is_self_delegation { // || delegator == validator {
//         //      // `normal scene` or `do self-delegation`
//         // } else {
//         //     return Err(eg!("self-delegation has not been finished"));
//         // }
//
//         // check delegation amount
//         voting::check_delegation_amount(amount, true)?;
//
//         // check power
//         StakingModule::check_validator_power(&validator, amount, context)
//     }
//
//     pub fn check_delegation_amount(am: Amount, is_append: bool) -> Result<()> {
//         let lowb = alt!(
//             is_append,
//             MIN_DELEGATION_AMOUNT,
//             STAKING_VALIDATOR_MIN_POWER
//         );
//         if (lowb..=MAX_DELEGATION_AMOUNT).contains(&am) {
//             return Ok(());
//         } else {
//             let msg = format!(
//                 "Invalid delegation amount: {} (min: {}, max: {})",
//                 am, lowb, MAX_DELEGATION_AMOUNT
//             );
//             Err(eg!(msg))
//         }
//     }
//
//     pub fn check_validator_power(
//         validator: &crypto::PublicKey,
//         pending_power: Amount,
//         context: &mut AContext<Stateless<Self>, Stateful<Self>>,
//     ) -> Result<()> {
//         // let global_power = self.global_delegation_amount.get() + pending_power;
//         let global_power = *context
//             .stateful
//             .global_delegation_amount
//             .get()
//             .unwrap()
//             .unwrap()
//             .deref()
//             + pending_power;
//
//         if MAX_TOTAL_POWER < global_power {
//             return Err(eg!("global power overflow"));
//         }
//
//         let key = ValidatorPublicKey::from_crypto_publickey(&validator).unwrap();
//         // let cur_power = *self.validator_power_map.get(&key).unwrap();
//         let cur_power = *context
//             .stateful
//             .validator_power_map
//             .get(&key)
//             .unwrap()
//             .unwrap()
//             .deref();
//
//         if ((cur_power + pending_power) as u128)
//             .checked_mul(MAX_POWER_PERCENT_PER_VALIDATOR[1])
//             .c(d!())?
//             > MAX_POWER_PERCENT_PER_VALIDATOR[0]
//                 .checked_mul(global_power as u128)
//                 .c(d!())?
//         {
//             return Err(eg!("validator power overflow"));
//         }
//
//         Ok(())
//     }
// }
