use crate::asset::AssetMeta;
use crate::evm;
use crate::rewards;
use crate::staking;
use crate::utxo;

#[derive(Debug, Clone)]
pub enum Operation {
    TransferAsset,
    DefineAsset(AssetMeta),
    IssueAsset,
    Fee,
    Delegate(staking::Delegate),
    ClaimReward(rewards::Claim),
    Undelegate(staking::Undelegate),
    EvmCall(evm::Evm),
}

#[derive(Debug, Clone)]
pub struct Output {
    pub core: utxo::Output,
    pub operation: Operation,
}
