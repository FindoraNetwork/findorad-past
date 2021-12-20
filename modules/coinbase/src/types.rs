use libfindora::utxo::Output;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OutputChain {
    pub output: Output,
    pub next: i64,
}
