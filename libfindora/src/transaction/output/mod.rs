use serde::{Deserialize, Serialize};
use zei::xfr::structs::BlindAssetRecord;

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    TransferAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub core: BlindAssetRecord,
    pub operation: Operation,
}
