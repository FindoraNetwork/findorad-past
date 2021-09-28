use serde::{Deserialize, Serialize};
use zei::xfr::structs::{BlindAssetRecord, OwnerMemo};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    pub core: BlindAssetRecord,
    pub operation: Operation,
    pub owner_memo: Option<OwnerMemo>,
}
