///! delegate operation logics
/// increase validator's voting power
///
use crate::{
    validator_keys::ValidatorPublicKey,
    voting::{global_power_rules, validator_power_rules},
};
use abcf::{
    bs3::{MapStore, ValueStore},
    tm_protos::abci::ValidatorUpdate,
    Error,
};
use libfindora::staking::voting::{
    Amount, Power, MAX_DELEGATION_AMOUNT, MIN_DELEGATION_AMOUNT, STAKING_VALIDATOR_MIN_POWER,
};
use libfindora::staking::TendermintAddress;
use std::collections::BTreeMap;
use zei::xfr::sig::XfrPublicKey;

/// delegation operation
pub struct DelegateOp {
    pub delegator: XfrPublicKey,
    pub validator_address: TendermintAddress,
    pub validator_pubkey: Option<ValidatorPublicKey>,
    pub amount: Amount,
    pub memo: Option<String>,
}

/// execute delegate operation
pub fn execute_delegate<'a>(
    op: DelegateOp,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<XfrPublicKey, Amount>,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<XfrPublicKey, Amount>>,
    powers: &mut impl MapStore<TendermintAddress, Power>,
    validator_staker: &mut impl MapStore<TendermintAddress, XfrPublicKey>,
    validator_addr_pubkey: &mut impl MapStore<TendermintAddress, ValidatorPublicKey>,
    delegation_info: &mut impl MapStore<XfrPublicKey, BTreeMap<TendermintAddress, Amount>>,
) -> abcf::Result<Vec<ValidatorUpdate>> {
    // op.validator exists && has done self-delegate operation
    if let Some(power) = powers.get_mut(&op.validator_address)? {
        if op.amount >= MIN_DELEGATION_AMOUNT && op.amount <= MAX_DELEGATION_AMOUNT {
            let current_power =
                power
                    .checked_add(op.amount)
                    .ok_or(abcf::Error::ABCIApplicationError(
                        90002,
                        "add validator power overflow".to_string(),
                    ))?;
            let mut current_global_power = 0;
            if let Some(p) = global_power.get()? {
                let power = p
                    .checked_add(op.amount)
                    .ok_or(abcf::Error::ABCIApplicationError(
                        90002,
                        "add global power overflow".to_string(),
                    ))?;

                current_global_power = power;
            }

            if global_power_rules(current_global_power)?
                && validator_power_rules(current_power, current_global_power)?
            {
                // update global power
                global_power.set(current_global_power)?;

                // update delegation_amount
                let actual_amount;
                if let Some(delegation_amount) = delegation_amount.get_mut(&op.delegator)? {
                    let amount = *delegation_amount;
                    let amount =
                        amount
                            .checked_add(op.amount)
                            .ok_or(abcf::Error::ABCIApplicationError(
                                90002,
                                "add all delegation amount overflow".to_string(),
                            ))?;
                    actual_amount = amount;
                    *delegation_amount = amount;
                } else {
                    // add new delegation amount
                    actual_amount = op.amount;
                    delegation_amount.insert(op.delegator, op.amount)?;
                }

                // update delegators
                if let Some(delegate) = delegators.get_mut(&op.validator_address)? {
                    delegate.insert(op.delegator, actual_amount);
                } else {
                    // add new delegator
                    let mut delegator = BTreeMap::new();
                    delegator.insert(op.delegator, actual_amount);
                    delegators.insert(op.validator_address.clone(), delegator)?;
                }

                // update powers
                powers.insert(op.validator_address.clone(), current_power)?;

                // update delegation_info
                if let Some(map) = delegation_info.get_mut(&op.delegator)? {
                    if let Some(amount) = map.get_mut(&op.validator_address) {
                        *amount = actual_amount;
                    } else {
                        map.insert(op.validator_address.clone(), actual_amount);
                    }
                } else {
                    let mut td_addr_amount_map = BTreeMap::new();
                    td_addr_amount_map.insert(op.validator_address.clone(), actual_amount);
                    delegation_info.insert(op.delegator.clone(), td_addr_amount_map.clone())?;
                }

                let pub_key = if let Some(pubkey) = &op.validator_pubkey {
                    pubkey.to_crypto_publickey()
                } else {
                    if let Some(pub_key) = validator_addr_pubkey.get(&op.validator_address)? {
                        pub_key.to_crypto_publickey()
                    } else {
                        return Err(abcf::Error::ABCIApplicationError(
                            90003,
                            "there is no matching public key for this address".to_string(),
                        ));
                    }
                };

                let validator_update = ValidatorUpdate {
                    pub_key,
                    power: current_power as i64,
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
                let power = p
                    .checked_add(op.amount)
                    .ok_or(abcf::Error::ABCIApplicationError(
                        90002,
                        "add global power overflow".to_string(),
                    ))?;
                current_global_power = power;
            }

            if global_power_rules(current_global_power)? {
                // update global power
                global_power.set(current_global_power)?;

                // udate powers
                powers.insert(op.validator_address.clone(), op.amount)?;

                // add address->xfr_pubkey, address->validator_pubkey
                validator_staker.insert(op.validator_address.clone(), op.delegator)?;

                // update delegation_info
                if let Some(map) = delegation_info.get_mut(&op.delegator)? {
                    if let Some(amount) = map.get_mut(&op.validator_address) {
                        *amount = op.amount;
                    } else {
                        map.insert(op.validator_address.clone(), op.amount);
                    }
                } else {
                    let mut td_addr_amount_map = BTreeMap::new();
                    td_addr_amount_map.insert(op.validator_address.clone(), op.amount);

                    delegation_info.insert(op.delegator.clone(), td_addr_amount_map)?;
                }

                let mut delegator = BTreeMap::new();
                delegator.insert(op.delegator, op.amount);
                delegators.insert(op.validator_address.clone(), delegator)?;

                // must safe ,this field must be present when self-delegation
                let validator_pk =  if let Some(validator_pk)= op.validator_pubkey.clone() {
                    validator_pk
                } else {
                    return Err(abcf::Error::ABCIApplicationError(
                        90004,
                        "must contain a validator pubkey".to_string(),
                    ));
                };

                validator_addr_pubkey.insert(op.validator_address.clone(), validator_pk.clone())?;


                // validator pubkey must be exist when self-delegation
                let validator_update = ValidatorUpdate {
                    pub_key: validator_pk.to_crypto_publickey(),
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
