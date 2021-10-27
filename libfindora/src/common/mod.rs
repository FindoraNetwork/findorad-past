use zei::{
    serialization::ZeiFromToBytes,
    xfr::{
        sig::{XfrSecretKey, XfrPublicKey},
        structs::{AssetType as ZeiAssetType, ASSET_TYPE_LENGTH},
    },
};
use serde::{Deserialize, Serialize};
use crate::utxo::{Input, Output};

pub type Issuances = Vec<(QueryTxOutPut, Option<String>)>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub struct AssetTypeCode {
    pub val: ZeiAssetType
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Commitment([u8; 32]);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetType {
    pub properties: Asset,
    pub(crate) digest: [u8; 32],
    pub(crate) units: u64,
    pub(crate) confidential_units: Commitment,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub asset_type_code: AssetTypeCode,
    pub issuer: XfrPublicKey,
    pub memo: String,
    pub confidential_memo:Option<String>,
    pub policy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefineAsset {
    pub pubkey: XfrPublicKey,
    pub body: Asset,
    pub signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryTxOutPut {
    pub id: Input,
    pub record: Output,
    pub lien: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateCommitmentData {
    pub txns_in_block_hash: String,
    pub previous_state_commitment: Option<String>,
    pub txo_count: u64,
    pub staking: Option<String>,
}