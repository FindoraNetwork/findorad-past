use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub nonce: u64,
    pub code: Vec<u8>,
}
