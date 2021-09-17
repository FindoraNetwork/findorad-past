pub mod asset;
pub mod coinbase;
pub mod transaction;

use abcf_utxo::{UTXOModule, Config};
use asset::{AssetCode, PublicKey};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct UTXOConfig {}

// TODO: use zei type
impl Config for UTXOConfig {
    type Address = u64;

    type Signature = u64;

    type AssetCode = u64;

    type PublicKey = u64;

    type OutputId = u64;
}

pub type UTXO<S> = UTXOModule<UTXOConfig, S>;
pub type UtxoTx = abcf_utxo::Transaction<UTXOConfig>;


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxoSID(pub u64);

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxOutPut {
    id: Option<TxoSID>,
    code: AssetCode,
    amount: u64,
    owner: PublicKey,
}

impl TxOutPut {
    pub fn new(code: AssetCode, amount: u64, owner: PublicKey) -> Self {
        TxOutPut {
            id: None,
            code,
            amount,
            owner,
        }
    }

    pub fn update_id(&mut self, id: TxoSID) {
        self.id = Some(id);
    }
}
