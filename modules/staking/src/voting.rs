//! Voting logics
//!
//! deletate    -> increase voting power
//! un-delegate -> decrease voting power
//!

use crate::{
    delegate::{execute_delegate, DelegateOp},
    undelegate::{execute_undelegate, UnDelegateOp},
    validator_pubkey::ValidatorPublicKey,
};
use abcf::{
    bs3::{MapStore, ValueStore},
    tm_protos::abci::ValidatorUpdate,
    Error,
};
use libfindora::staking::{
    voting::{Amount, Power, MAX_POWER_PERCENT_PER_VALIDATOR, MAX_TOTAL_POWER},
    Operation, StakingInfo,
};
use std::collections::BTreeMap;
use zei::xfr::sig::XfrPublicKey;

pub fn execute_staking(
    info: &StakingInfo,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<XfrPublicKey, Amount>,
    delegators: &mut impl MapStore<ValidatorPublicKey, BTreeMap<XfrPublicKey, Amount>>,
    powers: &mut impl MapStore<ValidatorPublicKey, Power>,
) -> abcf::Result<Vec<ValidatorUpdate>> {
    match &info.operation {
        Operation::Delegate(d) => {
            let op = DelegateOp {
                delegator: info.delegator.clone(),
                validator: ValidatorPublicKey::from_crypto_publickey(&d.validator).unwrap(),
                amount: info.amount,
                memo: d.memo.clone(),
            };

            return execute_delegate(op, global_power, delegation_amount, delegators, powers);
        }
        Operation::Undelegate(ud) => {
            let op = UnDelegateOp {
                delegator: info.delegator.clone(),
                validator: ValidatorPublicKey::from_crypto_publickey(&ud.validator).unwrap(),
                amount: info.amount,
            };

            return execute_undelegate(op, global_power, delegation_amount, delegators, powers);
        }
    }
}

/// check validatro's power rules
/// A single validator's power percentage MUST NOT > MAX_POWER_PERCENT_PER_VALIDATOR
pub fn validator_power_rules(
    current_power: Power,
    current_global_power: Power,
) -> abcf::Result<()> {
    if (current_power as u128)
        .checked_mul(MAX_POWER_PERCENT_PER_VALIDATOR[1])
        .unwrap()
        > MAX_POWER_PERCENT_PER_VALIDATOR[0]
            .checked_mul(current_global_power as u128)
            .unwrap()
    {
        return Err(Error::ABCIApplicationError(
            90001,
            "validator power overflow".to_owned(),
        ));
    }

    Ok(())
}

/// check global power rules
/// after delegate operation, new global power MUST NOT > MAX_TOTAL_POWER
pub fn global_power_rules(current_global_power: Power) -> abcf::Result<()> {
    if MAX_TOTAL_POWER < current_global_power {
        return Err(Error::ABCIApplicationError(
            90001,
            "global power overflow".to_owned(),
        ));
    }

    Ok(())
}
