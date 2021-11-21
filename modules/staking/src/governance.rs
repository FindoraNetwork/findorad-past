use abcf::bs3::{MapStore, ValueStore};
use abcf::tm_protos::abci::Validator;
use libfindora::staking::voting::{Amount, Power};
use libfindora::staking::TendermintAddress;
use libfindora::utxo::Address;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

    pub fn from_evidence_type(ty: i32) -> Self {
        match ty {
            0 => Self::Unknown,
            1 => Self::DuplicateVote,
            2 => Self::LightClientAttack,
            _ => {
                // Panic here, beacuse this error is caused by tendermint.
                panic!("Receive error evidence index number from tendermint.");
            }
        }
    }
}

pub fn penalty_amount_power<'a>(
    powers: &mut impl MapStore<TendermintAddress, Power>,
    global_power: &mut impl ValueStore<Power>,
    delegation_amount: &mut impl MapStore<Address, Amount>,
    delegators: &mut impl MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    validator_staker: &mut impl MapStore<TendermintAddress, Address>,
    penalty_list: &Vec<(Validator, ByzantineKind)>,
) {
    for (validator, bk) in penalty_list.iter() {
        // let v_addr = TendermintAddress::new_from_vec(&validator.address);
        let inner = validator
            .address
            .as_slice()
            .try_into()
            .expect("Tendermint error");
        let v_addr = TendermintAddress(inner);

        let v_xfr_pk =
            if let Some(pub_key) = validator_staker.get(&v_addr).expect("read storage error.") {
                pub_key
            } else {
                log::error!("there is no matching xfr pubkey for this address");
                return;
            };

        let pr = bk.penalty_rate();
        // penalty amount
        let mut pa = 0;

        // penalty the money to be fined based on the total amount of his self-delegation
        if let Some(delegate_map) = delegators.get_mut(&v_addr).expect("read storage error.") {
            if let Some(amount) = delegate_map.get_mut(&*v_xfr_pk) {
                pa = *amount * pr[0] / pr[1];
                *amount = amount.saturating_sub(pa);
            }
        }

        // penalize the total pledge amount of this verifier
        if let Some(delegation_amount) = delegation_amount
            .get_mut(&*v_xfr_pk)
            .expect("read storage error.")
        {
            *delegation_amount = delegation_amount.saturating_sub(pa);
        }

        // deducting the power of the validator
        if let Some(v_power) = powers.get_mut(&v_addr).expect("read storage error.") {
            *v_power = v_power.saturating_sub(pa);
        }

        // deducting the power of the global
        if let Some(g_power) = global_power.get().expect("read storage error.") {
            let g_cnt_power = g_power.saturating_sub(pa);
            global_power.set(g_cnt_power).expect("Set storage error.");
        }
    }
}
