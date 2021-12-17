use primitive_types::U256;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AssetMeta {
    pub maximum: Option<U256>,
    pub transferable: bool,
}
