use zei::xfr::structs::{BlindAssetRecord, OwnerMemo};

use crate::staking;
use crate::rewards;

#[derive(Debug, Clone)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
    Fee,
    Undelegate(staking::Delegate),
    ClaimReward(rewards::Claim),
    Delegate(staking::Undelegate),
}

#[derive(Debug)]
pub struct Output {
    pub core: BlindAssetRecord,
    pub operation: Operation,
    pub owner_memo: Option<OwnerMemo>,
}
