use libfindora::utxo::Output;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct OutputChain {
    pub output: Output,
    pub next: i64,
}
