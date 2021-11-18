use primitive_types::{H160, H256, U256};

use crate::transaction;

#[derive(Debug, Clone)]
pub struct Call {
    pub address: H160,
    pub value: U256,
}

#[derive(Debug, Clone)]
pub struct Create2 {
    pub salt: H256,
}

#[derive(Debug, Clone)]
pub enum Action {
    Call(Call),
    Create,
    Create2(Create2),
}

impl Default for Action {
    fn default() -> Self {
        Self::Create
    }
}

#[derive(Debug, Clone, Default)]
pub struct EvmCall {
    pub caller: H160,
    pub data: Vec<u8>,
    pub gas_limit: u64,
    pub action: Action,
}

#[derive(Debug, Clone, Default)]
pub struct EvmTransaction {
    pub calls: Vec<EvmCall>,
}

impl TryFrom<transaction::Transaction> for EvmTransaction {
    type Error = abcf::Error;

    fn try_from(_tx: transaction::Transaction) -> Result<EvmTransaction, Self::Error> {
        let calls = Vec::new();

        // TODO: check signature and match for ethereum memo.

        Ok(EvmTransaction {
            calls
        })
    }
}

