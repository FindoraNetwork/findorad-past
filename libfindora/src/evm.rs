use primitive_types::H256;
use serde::{Deserialize, Serialize};

use crate::Address;

#[derive(Debug, Clone)]
pub struct EvmMemo {
    pub tx: Vec<u8>,
    pub n: u32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Create2 {
    pub salt: H256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Call,
    Create,
    Create2(Create2),
}

impl Default for Action {
    fn default() -> Self {
        Action::Call
    }
}

#[derive(Debug, Clone, Default)]
pub struct Evm {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub data: Vec<u8>,
    pub action: Action,
    pub caller: Address,
}
