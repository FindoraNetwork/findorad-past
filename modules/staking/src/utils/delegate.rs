use std::collections::BTreeMap;

use abcf::bs3::{MapStore, ValueStore};
use libfindora::{
    asset::Amount,
    staking::{Delegate, TendermintAddress, ValidatorPublicKey},
    Address,
};

use crate::{Error, Power, Result, FRA_STAKING};

pub fn apply_delegated(
    delegator: &Address,
    op: &Delegate,
    validator_staker: &mut impl MapStore<TendermintAddress, Address>,
    validator_pubkey: &mut impl MapStore<TendermintAddress, ValidatorPublicKey>,
) -> Result<ValidatorPublicKey> {
    let pubkey = if let Some(key) = &op.validator {
        let result = validator_pubkey.insert(op.address.clone(), key.clone())?;
        if result.is_some() {
            return Err(Error::AlreadySelfDelegate);
        }

        key.clone()
    } else {
        validator_pubkey
            .get(&op.address)?
            .ok_or(Error::MustDoSelfDegegateFirst)?
            .clone()
    };

    validator_staker.insert(op.address.clone(), delegator.clone())?;
    Ok(pubkey)
}

pub fn apply_global(
    amount: Amount,
    op: &Delegate,
    global_power: &mut impl ValueStore<Power>,
    powers: &mut impl MapStore<TendermintAddress, Power>,
) -> Result<Power> {
    // Check amount is in range.
    if op.validator.is_some()
        && amount <= FRA_STAKING.validator_min_power
        && amount >= FRA_STAKING.max_delegate()
    {
        return Err(Error::DelegateAmountOutOfRange(
            FRA_STAKING.validator_min_power,
            FRA_STAKING.max_delegate(),
        ));
    }
    if amount <= FRA_STAKING.min_delegate && amount >= FRA_STAKING.max_delegate() {
        return Err(Error::DelegateAmountOutOfRange(
            FRA_STAKING.min_delegate,
            FRA_STAKING.max_delegate(),
        ));
    }

    // Global power.
    let result_global_power = {
        // Compute global_power.
        let result_global_power = if let Some(gp) = global_power.get()? {
            gp.checked_add(amount).ok_or(Error::OverflowAdd)?
        } else {
            0
        };

        if result_global_power >= FRA_STAKING.max_delegate() {
            return Err(Error::DelegateAmountOutOfRange(
                FRA_STAKING.min_delegate,
                FRA_STAKING.max_delegate(),
            ));
        }

        // Pass check, set value.
        global_power.set(result_global_power)?;

        result_global_power
    };

    let tp = {
        // Get validator's power.
        let result_power = if let Some(power) = powers.get(&op.address)? {
            power.checked_add(amount).ok_or(Error::OverflowAdd)?
        } else {
            amount
        };

        if result_global_power != 0 {
            let max_delegate_global = (result_global_power as u128)
                .checked_mul(FRA_STAKING.max_percent_per_validator[0] as u128)
                .ok_or(Error::OverflowAdd)?;
            let max_delegate_current = (result_power as u128)
                .checked_mul(FRA_STAKING.max_percent_per_validator[1] as u128)
                .ok_or(Error::OverflowAdd)?;

            if max_delegate_current > max_delegate_global {
                return Err(Error::DelegateAmountOutOfRange(
                    0,
                    max_delegate_global
                        .overflowing_div(FRA_STAKING.max_percent_per_validator[1] as u128)
                        .0
                        .try_into()
                        .unwrap_or_default(),
                ));
            }
        } else {
            global_power.set(result_power)?;
        }

        // set value
        powers.insert(op.address.clone(), result_power)?;
        result_power
    };

    Ok(tp)
}

pub fn apply_detail(
    delegator: &Address,
    amount: Amount,
    op: &Delegate,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    delegation_amount: &mut impl MapStore<Address, Amount>,
) -> Result<()> {
    // increase in delegator amount
    if let Some(a) = delegation_amount.get_mut(delegator)? {
        *a += amount;
    } else {
        delegation_amount.insert(delegator.clone(), amount)?;
    }

    // increase the amount of delegator under validator
    if let Some(v) = delegators.get_mut(&op.address)? {
        if let Some(a) = v.get_mut(delegator) {
            *a += amount;
        } else {
            v.insert(delegator.clone(), amount);
        }
    } else {
        let mut v = BTreeMap::new();

        v.insert(delegator.clone(), amount);

        delegators.insert(op.address.clone(), v)?;
    }

    Ok(())
}
