use primitive_types::H160;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleVersionRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleVersionResponse {
    version: Option<H160>,
}
