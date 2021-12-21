use std::collections::BTreeMap;

use abcf::bs3::{MapStore, ValueStore};
use libfindora::{
    asset::Amount,
    staking::{TendermintAddress, Undelegate},
    Address,
};

use crate::{Error, Power, Result};

pub fn apply_undelegate_amount(
    amount: Amount,
    delegator: &Address,
    op: &Undelegate,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    global_power: &mut impl ValueStore<Power>,
    powers: &mut impl MapStore<TendermintAddress, Power>,
    delegation_amount: &mut impl MapStore<Address, Amount>,
) -> Result<()> {
    if let Some(v) = delegators.get_mut(&op.address)? {
        if let Some(a) = v.get_mut(delegator) {
            let undelegate_amount = a
                .checked_sub(amount)
                .ok_or_else(|| Error::DelegateAmountNotEnough)?;

            *a = undelegate_amount;
        } else {
            return Err(Error::DelegateAmountNotEnough);
        }
    } else {
        return Err(Error::DelegateAmountNotEnough);
    }

    if let Some(v) = global_power.get()? {
        let a = v
            .checked_sub(amount)
            .ok_or_else(|| Error::DelegateAmountNotEnough)?;
        global_power.set(a)?;
    } else {
        return Err(Error::DelegateAmountNotEnough);
    }

    if let Some(v) = powers.get_mut(&op.address)? {
        *v = v
            .checked_sub(amount)
            .ok_or_else(|| Error::DelegateAmountNotEnough)?;
    } else {
        return Err(Error::DelegateAmountNotEnough);
    }

    if let Some(v) = delegation_amount.get_mut(delegator)? {
        *v = v
            .checked_sub(amount)
            .ok_or_else(|| Error::DelegateAmountNotEnough)?;
    } else {
        return Err(Error::DelegateAmountNotEnough);
    }

    Ok(())
}
