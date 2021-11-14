use primitive_types::{H160, H256, U256};

pub struct Call {
    pub address: H160,
    pub value: U256,
}

pub struct Create2 {
    pub salt: H256,
}

pub enum Action {
    Call(Call),
    Create,
    Create2(Create2),
    Transfer(Call),
}

pub struct EthereumTransaction {
    pub caller: H160,
    pub data: Vec<u8>,
    pub gas_limit: u64,
    pub action: Action,
}

impl Default for EthereumTransaction {
    fn default() -> Self {
        let action = Call {
            address: H160::default(),
            value: U256::default(),
        };

        Self {
            caller: H160::default(),
            data: Vec::new(),
            gas_limit: 0,
            action: Action::Transfer(action),
        }
    }
}

