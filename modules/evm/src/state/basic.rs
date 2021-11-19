use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountBasic {
    pub nonce: U256,
    pub code: Vec<u8>,
}
