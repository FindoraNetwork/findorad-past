use primitive_types::{H160, U256};

#[derive(Debug, Clone)]
pub struct InputOperation {
    pub caller: H160,
    pub amount: U256,
}

#[derive(Debug, Clone)]
pub struct OutputOperation {
    pub target: H160,
    pub amount: U256,
}
