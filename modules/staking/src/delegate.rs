use abcf::{
    bs3::{MapStore, ValueStore},
    tm_protos::abci::ValidatorUpdate,
    Error,
};
use libfindora::staking::voting::{Amount, Power, MAX_DELEGATION_AMOUNT, MIN_DELEGATION_AMOUNT};
use zei::xfr::sig::XfrPublicKey;

use crate::ValidatorPublicKey;

#[allow(dead_code)]
pub struct DelegateOp {
    delegator: XfrPublicKey,
    validator: ValidatorPublicKey,
    amount: Amount,
    memo: Option<String>,
}

// pub struct DelegateContext<'a> {
// powers: &'a mut dyn MapStore<ValidatorPublicKey, Power>,
// }

#[allow(dead_code)]
pub fn execute_delegate<'a>(
    op: DelegateOp,
    powers: &mut impl MapStore<ValidatorPublicKey, Power>,
    _global_power: &mut impl ValueStore<Power>,
) -> abcf::Result<Vec<ValidatorUpdate>> {
    if let Some(power) = powers.get_mut(&op.validator)? {
        // is MIN_DELEGATION_AMOUNT ~ MAX_DELEGATION_AMOUNT
        if op.amount >= MIN_DELEGATION_AMOUNT && op.amount <= MAX_DELEGATION_AMOUNT {
            *power += op.amount;

            let global_power = 0;

            if *power > global_power {
                return Err(Error::ABCIApplicationError(90001, String::from("Delege")));
            }
            //             let current = global_power.get()?;
            //             global_power.set(current + *power);
            // *global_power += op.amount;
        }
    } else {
        // is MIN_STAKING_AMOUNT ~ MAX_DELEGATION_AMOUNT
    }

    Ok(Vec::new())
}
