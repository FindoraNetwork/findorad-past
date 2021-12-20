use std::collections::BTreeMap;

use abcf::bs3::{MapStore, ValueStore};
use libfindora::{
    asset::Amount,
    staking::{TendermintAddress, ValidatorPublicKey},
    Address,
};

use crate::{Error, Power, Result};

use super::{BlockEvidence, ByzantineKind};

fn compute_penalty_amount(amount: Amount, rate: [u64; 2]) -> Option<Amount> {
    let upper = amount.checked_mul(rate[0])?;

    Some(upper.checked_div(rate[1])?)
}

pub fn penalty_single(
    evidence_validator_address: &TendermintAddress,
    kind: ByzantineKind,
    validator_staker: &mut impl MapStore<TendermintAddress, Address>,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    validator_powers: &mut impl MapStore<TendermintAddress, Power>,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<Address, Amount>,
) -> Result<i64> {
    let staker_address = validator_staker
        .get(evidence_validator_address)?
        .ok_or_else(|| Error::IsOptionNone)?;

    // process delegation detail.
    let delegator_map = delegators
        .get_mut(evidence_validator_address)?
        .ok_or_else(|| Error::IsOptionNone)?;

    let amount = delegator_map
        .get_mut(&staker_address)
        .ok_or_else(|| Error::IsOptionNone)?;

    let rate = kind.penalty_rate();

    let penalty_power = compute_penalty_amount(*amount, rate).unwrap_or_default();

    *amount = amount.checked_sub(penalty_power).unwrap_or_default();

    // process validator.
    let amount = validator_powers
        .get_mut(evidence_validator_address)?
        .ok_or_else(|| Error::IsOptionNone)?;

    let power = amount.checked_sub(penalty_power).unwrap_or_default();
    *amount = power;

    let amount = global_power.get()?.ok_or_else(|| Error::IsOptionNone)?;

    // process global_power.
    let a = amount.checked_sub(penalty_power).unwrap_or_default();

    global_power.set(a)?;

    // process delegator.
    let amount = delegation_amount
        .get_mut(&staker_address)?
        .ok_or_else(|| Error::IsOptionNone)?;
    *amount = amount.checked_sub(penalty_power).unwrap_or_default();

    return Ok(power.try_into()?);
}

pub fn penalty(
    evidences: &BlockEvidence,
    // validator_pubkey: &mut impl MapStore<TendermintAddress, ValidatorPublicKey>,
    validator_powers: &mut impl MapStore<TendermintAddress, Power>,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<Address, Amount>,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    validator_staker: &mut impl MapStore<TendermintAddress, Address>,
    validator_pubkey: &mut impl MapStore<TendermintAddress, ValidatorPublicKey>,
) -> Result<BTreeMap<ValidatorPublicKey, i64>> {
    let mut res = BTreeMap::new();

    for evidence in &evidences.evidences {
        let evidence_validator_address = evidence
            .validator
            .as_ref()
            .ok_or_else(|| Error::NoTendermintAddress)?;

        let power = penalty_single(
            evidence_validator_address,
            evidence.kind.clone(),
            validator_staker,
            delegators,
            validator_powers,
            global_power,
            delegation_amount,
        )?;

        if let Some(pubkey) = validator_pubkey.get(evidence_validator_address)? {
            res.insert(pubkey.clone(), power);
        }
    }

    Ok(res)
}
