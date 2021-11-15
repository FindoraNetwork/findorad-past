///! un-delegate operation logics
/// decrease validator's voting power
///
use crate::validator_keys::ValidatorPublicKey;
use abcf::{
    bs3::{MapStore, ValueStore},
    tm_protos::abci::ValidatorUpdate,
    Error,
};
use libfindora::staking::voting::{Amount, Power};
use std::collections::BTreeMap;
use zei::xfr::sig::XfrPublicKey;

/// un-delegate operation
pub struct UnDelegateOp {
    pub delegator: XfrPublicKey,
    pub validator: ValidatorPublicKey,
    pub amount: Amount,
}

/// execute un-delegate operation
pub fn execute_undelegate<'a>(
    op: UnDelegateOp,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<XfrPublicKey, Amount>,
    delegators: &mut impl MapStore<ValidatorPublicKey, BTreeMap<XfrPublicKey, Amount>>,
    powers: &mut impl MapStore<ValidatorPublicKey, Power>,
) -> abcf::Result<Vec<ValidatorUpdate>> {
    if let Some(power) = powers.get_mut(&op.validator)? {
        // undelegate from validator
        if let Some(delegate_map) = delegators.get_mut(&op.validator)? {
            if let Some(amount) = delegate_map.get_mut(&op.delegator) {
                if *amount >= op.amount {
                    // delegate op
                    let amount = *amount;

                    // update global power
                    let mut current_global_power = 0;
                    if let Some(p) = global_power.get()? {
                        let power = p.checked_sub(op.amount).ok_or(
                            abcf::Error::ABCIApplicationError(90002, "sub global power overflow".to_string())
                        )?;
                        current_global_power = power;
                    }
                    global_power.set(current_global_power)?;

                    // update delegation_amount
                    let current_delegatioon_amount =
                        amount.checked_sub(op.amount).ok_or(
                            abcf::Error::ABCIApplicationError(90002,
                                                              "sub delegation all amount overflow".to_string())
                        )?;
                    delegation_amount.insert(op.delegator.clone(), current_delegatioon_amount)?;

                    // update delegators
                    delegate_map
                        .insert(op.delegator.clone(), current_delegatioon_amount)
                        .ok_or(amount)
                        .unwrap();

                    // update powers
                    let current_power = power.checked_sub(op.amount).ok_or(
                        abcf::Error::ABCIApplicationError(90002,
                                                          "sub validator power overflow".to_string())
                    )?;
                    powers.insert(op.validator.clone(), current_power)?;

                    let validator_update = ValidatorUpdate {
                        pub_key: op.validator.to_crypto_publickey(),
                        power: current_power as i64,
                    };
    
                    return Ok(vec![validator_update]);    
                } else {
                    // op.amount > delegated amount
                    let msg = format!(
                        "undelegated amount {} > delegated amount {}",
                        op.amount, *amount
                    );
                    return Err(Error::ABCIApplicationError(90001, msg));
                }
            } else {
                // op.delegator doesn't delegated to this validator
                let msg = format!(
                    "delegator {:?} doesn't delegated to validator {:?}",
                    op.delegator, op.validator
                );
                return Err(Error::ABCIApplicationError(90001, msg));
            }
        }
    } else {
        // validator doesn't exists
        let msg = format!("Validator {:?} doesn't exists", op.validator);
        return Err(Error::ABCIApplicationError(90001, msg));
    }

    Ok(Vec::new())
}
