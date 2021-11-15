use crate::validator_keys::{ValidatorAddr, ValidatorPubKeyPair, ValidatorPublicKey};
use abcf::bs3::{MapStore, ValueStore};
use abcf::tm_protos::abci::Validator;
use libfindora::staking::voting::{Amount, Power};
use ruc::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use zei::xfr::sig::XfrPublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ByzantineKind {
    DuplicateVote,
    LightClientAttack,
    OffLine,
    Unknown,
}

impl ByzantineKind {
    pub fn penalty_rate(&self) -> [u64; 2] {
        return match self {
            ByzantineKind::DuplicateVote => [5, 100],
            ByzantineKind::LightClientAttack => [1, 100],
            ByzantineKind::OffLine => [1, 1000_0000],
            ByzantineKind::Unknown => [30, 100],
        };
    }

    pub fn from_evidence_type(ty: i32) -> Result<Self> {
        match ty {
            0 => Ok(Self::Unknown),
            1 => Ok(Self::DuplicateVote),
            2 => Ok(Self::LightClientAttack),
            _ => Err(Box::from(d!("Failed to match for evidence type"))),
        }
    }
}

pub fn penalty_amount_power<'a>(
    powers: &mut impl MapStore<ValidatorPublicKey, Power>,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<XfrPublicKey, Amount>,
    delegators: &mut impl MapStore<ValidatorPublicKey, BTreeMap<XfrPublicKey, Amount>>,
    validator_addr_map: &mut impl MapStore<ValidatorAddr, ValidatorPubKeyPair>,
    penalty_list: &Vec<(Validator, ByzantineKind)>,
) -> abcf::Result<()> {
    for (validator, bk) in penalty_list.iter() {
        let v_addr = ValidatorAddr {
            addr: validator.address.clone(),
        };

        if let Some(v_key_pair) = validator_addr_map.get(&v_addr)? {
            let v_addr_pk = &v_key_pair.0;
            let v_xfr_pk = &v_key_pair.1;

            let pr = bk.penalty_rate();
            // penalty amount
            let mut pa = 0;

            // penalty the money to be fined based on the total amount of his self-delegation
            if let Some(delegate_map) = delegators.get_mut(v_addr_pk)? {
                if let Some(amount) = delegate_map.get_mut(v_xfr_pk) {
                    pa = *amount * pr[0] / pr[1];
                    *amount = amount.saturating_sub(pa);
                }
            }

            // penalize the total pledge amount of this verifier
            if let Some(delegation_amount) = delegation_amount.get_mut(v_xfr_pk)? {
                *delegation_amount = delegation_amount.saturating_sub(pa);
            }

            // deducting the power of the validator
            if let Some(v_power) = powers.get_mut(v_addr_pk)? {
                *v_power = v_power.saturating_sub(pa);
            }

            // deducting the power of the global
            if let Some(g_power) = global_power.get()? {
                let g_cnt_power = g_power.saturating_sub(pa);
                global_power.set(g_cnt_power)?;
            }
        }
    }

    Ok(())
}
