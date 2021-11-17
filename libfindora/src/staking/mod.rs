mod delegate;
pub use delegate::Delegate;
use serde::{Deserialize, Serialize};
mod undelegate;
pub use undelegate::Undelegate;

pub mod rpc;
pub mod voting;

use crate::{transaction, FRA_XFR_ASSET_TYPE};
use lazy_static::lazy_static;
use ruc::{pnk, RucResult};
use std::convert::TryFrom;
use zei::{
    serialization::ZeiFromToBytes,
    xfr::{sig::XfrPublicKey, structs::XfrAmount},
};

lazy_static! {
    pub static ref BLACK_HOLE_PUBKEY_STAKING: XfrPublicKey = pnk!(XfrPublicKey::zei_from_bytes(
        &[1; ed25519_dalek::PUBLIC_KEY_LENGTH][..]
    ));
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TendermintAddress([u8; 20]);

impl TendermintAddress {
    pub fn new_from_vec(v: &Vec<u8>) -> Self {
        let mut array = [0; 20];
        for (index, val) in v.iter().enumerate() {
            array[index] = *val;
        }
        Self { 0: array }
    }

    pub fn to_byte(&self) -> Vec<u8> {
        self.0.to_vec()
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    Delegate(Delegate),
    Undelegate(Undelegate),
}

#[derive(Debug, Clone)]
pub struct StakingInfo {
    pub delegator: XfrPublicKey,
    pub amount: u64,
    pub operation: Operation,
}

pub struct Transaction {
    pub infos: Vec<StakingInfo>,
}

impl Default for Transaction {
    fn default() -> Self {
        Self { infos: Vec::new() }
    }
}

impl TryFrom<&transaction::Transaction> for Transaction {
    type Error = abcf::Error;

    fn try_from(tx: &transaction::Transaction) -> Result<Transaction, Self::Error> {
        macro_rules! insert_info {
            ($infos:ident, $output:ident, $op:ident, $ty:ident) => {
                if $output.core.asset_type != FRA_XFR_ASSET_TYPE {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("Undelegate asset type must be FRA"),
                    ));
                }

                let delegator = $output.core.public_key.clone();
                let amount = if let XfrAmount::NonConfidential(v) = $output.core.amount {
                    v
                } else {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("Undelegate amount must be Non-confidential"),
                    ));
                };
                let operation = Operation::$ty($op.clone());

                let info = StakingInfo {
                    delegator,
                    amount,
                    operation,
                };

                $infos.push(info);
            };
        }

        let mut infos = Vec::new();

        for output in &tx.outputs {
            match &output.operation {
                transaction::OutputOperation::Undelegate(op) => {
                    insert_info!(infos, output, op, Undelegate);
                }
                transaction::OutputOperation::Delegate(op) => {
                    insert_info!(infos, output, op, Delegate);
                }
                _ => {}
            }
        }

        Ok(Transaction { infos })
    }
}
