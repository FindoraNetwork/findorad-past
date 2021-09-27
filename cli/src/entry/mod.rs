use zei::xfr::sig::{XfrKeyPair, XfrPublicKey};
use serde::{Deserialize, Serialize};
use zei::xfr::structs::XfrAssetType;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueBatchEntry {
    pub keypair: XfrKeyPair,
    pub amount: u64,
    pub asset_type: XfrAssetType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferBatchEntry {
    pub from: XfrKeyPair,
    pub to: XfrPublicKey,
    pub amount: u64,
    pub asset_type: XfrAssetType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceEntry {
    pub amount: u64,
    pub balance: u64,
}