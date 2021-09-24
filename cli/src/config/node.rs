use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub address: String,
    pub home: PathBuf,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            address: String::from("http://localhost:25576"),
            home: PathBuf::from(concat!(env!("HOME"), "/.findora/fn")),
        }
    }
}
