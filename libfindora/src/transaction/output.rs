use zei::xfr::structs::{BlindAssetRecord, OwnerMemo};

use crate::account;
use crate::evm;
use crate::rewards;
use crate::staking;

#[derive(Debug, Clone)]
pub enum Operation {
    TransferAsset,
    IssueAsset,
    Fee,
    Delegate(staking::Delegate),
    ClaimReward(rewards::Claim),
    Undelegate(staking::Undelegate),
    TransferAccount(account::OutputOperation),
    CallEVM(evm::EvmCall),
}

#[derive(Debug)]
pub struct Output {
    pub core: BlindAssetRecord,
    pub operation: Operation,
    pub owner_memo: Option<OwnerMemo>,
}
