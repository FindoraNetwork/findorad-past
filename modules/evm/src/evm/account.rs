use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    pub nonce: u64,
    pub code: Option<Vec<u8>>,
    pub reset: bool,
}
