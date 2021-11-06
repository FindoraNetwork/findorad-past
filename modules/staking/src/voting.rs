//! Voting logics
//!
//! deletate    -> increase voting power
//! un-delegate -> decrease voting power
//!

use abcf::tm_protos::abci::ValidatorUpdate;
use libfindora::staking::{Operation, StakingInfo};
// use ruc::*;

pub fn execute_staking(info: &StakingInfo) -> abcf::Result<Vec<ValidatorUpdate>> {
    let updates = Vec::new();

    match &info.operation {
        Operation::Delegate(_d) => {}
        Operation::Undelegate(_d) => {}
    }

    Ok(updates)
}

// /// Check amount can delegation?.
// pub fn check_delegation_amount(am: Amount, is_append: bool) -> Result<()> {
//     let lowb = alt!(
//         is_append,
//         MIN_DELEGATION_AMOUNT,
//         STAKING_VALIDATOR_MIN_POWER
//     );
//     if (lowb..=MAX_DELEGATION_AMOUNT).contains(&am) {
//         return Ok(());
//     } else {
//         let msg = format!(
//             "Invalid delegation amount: {} (min: {}, max: {})",
//             am, lowb, MAX_DELEGATION_AMOUNT
//         );
//         Err(eg!(msg))
//     }
// }
