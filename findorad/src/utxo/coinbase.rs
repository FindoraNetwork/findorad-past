use crate::utxo::{
    AssetCode, PublicKey,
    TxOutPut,
};

pub fn mint(miner: PublicKey, asset: AssetCode, amount: u64) -> TxOutPut {
    TxOutPut::new(asset, amount, miner)
}

pub fn burn(
    _owner: PublicKey,
    _asset: AssetCode,
    _amount: u64,
    _txos: &Vec<TxOutPut>,
) -> Vec<TxOutPut> {
    vec![]
}

// Stake/Bond
//      burn -> Valid TxoSid

// Reward/Unstake/Unbond
