use ruc::*;
use serde::{Deserialize, Serialize};

use crate::types::Wallet;

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletEntry {
    pub mnemonic: Option<String>,
}

impl WalletEntry {
    pub fn build_wallet(&self) -> Result<Wallet> {
        match &self.mnemonic {
            None => Wallet::generate(),
            Some(s) => Wallet::from_mnemonic(&s),
        }
    }
}
