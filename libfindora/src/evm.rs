use primitive_types::H256;

use crate::asset::Amount;

#[derive(Debug, Clone)]
pub struct Create2 {
    pub salt: H256,
}

#[derive(Debug, Clone)]
pub enum Action {
    Call,
    Create,
    Create2(Create2),
}

#[derive(Debug, Clone)]
pub struct Evm {
    pub nonce: u64,
    pub gas_limit: Amount,
    pub data: Vec<u8>,
    pub action: Action,
}
