pub mod asset;
pub mod coinbase;
pub mod transaction;

use abcf_utxo::{Config, UTXOModule};
use asset::AssetCode;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use zei::xfr::sig::{XfrKeyPair as KeyPair, XfrPublicKey as PublicKey, XfrSignature as Signature};

pub type OutputId = u64;
pub type Amount = u64;

#[derive(Debug, Clone)]
pub struct UTXOConfig {}

impl Config for UTXOConfig {
    type Address = PublicKey;

    type Signature = Signature;

    type AssetCode = AssetCode;

    type PublicKey = PublicKey;

    type OutputId = OutputId;
}

pub type UTXO<S> = UTXOModule<UTXOConfig, S>;
pub type UtxoTx = abcf_utxo::Transaction<UTXOConfig>;

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct TxOutPut {
    id: Option<OutputId>,
    code: AssetCode,
    amount: Amount,
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

    pub fn update_id(&mut self, id: OutputId) {
        self.id = Some(id);
    }
}
