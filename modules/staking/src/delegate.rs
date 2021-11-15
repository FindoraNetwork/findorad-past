///! delegate operation logics
/// increase validator's voting power
///
use crate::{
    voting::{global_power_rules, validator_power_rules},
    validator_keys::{ValidatorAddr, ValidatorPubKeyPair, ValidatorPublicKey},
};
use abcf::{
    bs3::{MapStore, ValueStore},
    tm_protos::abci::ValidatorUpdate,
    Error,
};
use libfindora::staking::voting::{
    Amount, Power, MAX_DELEGATION_AMOUNT, MIN_DELEGATION_AMOUNT, STAKING_VALIDATOR_MIN_POWER,
};
use std::collections::BTreeMap;
use zei::xfr::sig::XfrPublicKey;

/// delegation operation
pub struct DelegateOp {
    pub delegator: XfrPublicKey,
    pub validator: ValidatorPublicKey,
    pub amount: Amount,
    pub memo: Option<String>,
}

/// execute delegate operation
pub fn execute_delegate<'a>(
    op: DelegateOp,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<XfrPublicKey, Amount>,
    delegators: &mut impl MapStore<ValidatorPublicKey, BTreeMap<XfrPublicKey, Amount>>,
    powers: &mut impl MapStore<ValidatorPublicKey, Power>,
    validator_addr_map: &mut impl MapStore<ValidatorAddr, ValidatorPubKeyPair>,
) -> abcf::Result<Vec<ValidatorUpdate>> {
    // op.validator exists && has done self-delegate operation
    if let Some(power) = powers.get_mut(&op.validator)? {
        if op.amount >= MIN_DELEGATION_AMOUNT && op.amount <= MAX_DELEGATION_AMOUNT {
            let curren_power = power.checked_add(op.amount).ok_or(
                abcf::Error::ABCIApplicationError(90002, "add validator power overflow".to_string())
            )?;
            let mut current_global_power = 0;
            if let Some(p) = global_power.get()? {
                let power = p.checked_add(op.amount).ok_or(
                    abcf::Error::ABCIApplicationError(90002, "add global power overflow".to_string())
                )?;

                current_global_power = power;
            }

            if global_power_rules(current_global_power)?
                && validator_power_rules(curren_power, current_global_power)?
            {
                // update global power
                global_power.set(current_global_power)?;

                // update delegation_amount
                let actual_amount;
                if let Some(delegation_amount) = delegation_amount.get_mut(&op.delegator)? {
                    let amount = *delegation_amount;
                    let amount = amount.checked_add(op.amount).ok_or(
                        abcf::Error::ABCIApplicationError(90002, "add all delegation amount overflow".to_string())
                    )?;
                    actual_amount = amount;
                    *delegation_amount = amount;
                } else {
                    // add new delegation amount
                    actual_amount = op.amount;
                    delegation_amount.insert(op.delegator, op.amount)?;
                }

                // update delegators
                if let Some(delegate) = delegators.get_mut(&op.validator)? {
                    delegate.insert(op.delegator, actual_amount);
                } else {
                    // add new delegator
                    let mut delegator = BTreeMap::new();
                    delegator.insert(op.delegator, actual_amount);
                    delegators.insert(op.validator.clone(), delegator)?;
                }

                // update powers
                powers.insert(op.validator.clone(), curren_power)?;

                let validator_update = ValidatorUpdate {
                    pub_key: op.validator.to_crypto_publickey(),
                    power: curren_power as i64,
                };

                return Ok(vec![validator_update]);
            }

            return Ok(Vec::new());
        } else {
            let msg = format!(
                "Invalid delegation amount: {} (min: {}, max: {})",
                op.amount, MIN_DELEGATION_AMOUNT, MAX_DELEGATION_AMOUNT
            );
            return Err(Error::ABCIApplicationError(90001, msg));
        }
    } else {
        //execute self delegation
        if op.amount >= STAKING_VALIDATOR_MIN_POWER && op.amount <= MAX_DELEGATION_AMOUNT {
            let mut current_global_power = 0;
            if let Some(p) = global_power.get()? {
                let power = p.checked_add(op.amount).ok_or(
                    abcf::Error::ABCIApplicationError(90002, "add global power overflow".to_string())
                )?;
                current_global_power = power;
            }

            if global_power_rules(current_global_power)? {
                // update global power
                global_power.set(current_global_power)?;

                // udate powers
                powers.insert(op.validator.clone(), op.amount)?;

                // add validator addr
                validator_addr_map.insert(
                    op.validator.to_validator_addr(),
                    (op.validator.clone(), op.delegator),
                )?;

                let validator_update = ValidatorUpdate {
                    pub_key: op.validator.to_crypto_publickey(),
                    power: op.amount as i64,
                };

                return Ok(vec![validator_update]);
            }
        } else {
            return Err(Error::ABCIApplicationError(
                90001,
                "self-delegation has not been finished".to_owned(),
            ));
        }
    }

    Ok(Vec::new())
}
