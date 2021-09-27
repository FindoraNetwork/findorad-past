use libfindora::transaction::{Input, Output, Transaction};
use zei::xfr::sig::{XfrKeyPair, XfrPublicKey};
use serde::{Deserialize, Serialize};
use zei::xfr::structs::XfrAssetType;
use ruc::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueEntry {
    pub keypair: XfrKeyPair,
    pub amount: u64,
    pub asset_type: XfrAssetType,
    pub confidential_amount: bool,
}

impl IssueEntry {
    pub fn build(self) -> (Vec<Input>, Vec<Output>) {
        let inputs = Vec::new();
        let outputs = Vec::new();

        (inputs, outputs)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferEntry {
    pub from: XfrKeyPair,
    pub to: XfrPublicKey,
    pub amount: u64,
    pub asset_type: XfrAssetType,
    pub confidential_amount: bool,
    pub confidential_asset: bool,
}

impl TransferEntry {
    pub fn build(self) -> (Vec<Input>, Vec<Output>) {
        let inputs = Vec::new();
        let outputs = Vec::new();

        (inputs, outputs)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Issue(IssueEntry),
    Transfer(TransferEntry),
}

pub fn build_transaction(entries: Vec<Entry>) -> Result<Transaction> {
    let tx = Transaction::default();

    for entry in entries {
        let (inputs, outputs) = match entry {
            Entry::Issue(e) => e.build(),
            Entry::Transfer(e) => e.build(),
        };
    }

    Ok(tx)
}

