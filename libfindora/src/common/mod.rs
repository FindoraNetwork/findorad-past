use std::convert::TryFrom;
use zei::{
    xfr::{
        sig::XfrPublicKey,
        structs::{AssetType as ZeiAssetType, ASSET_TYPE_LENGTH},
    },
};
use serde::{Deserialize, Serialize};
use crate::utxo::{Input, Output};
use crate::transaction::{Output as tx_output, Input as tx_input};
use ruc::*;

pub type Issuances = Vec<(QueryTxOutPut, Option<String>)>;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Ord, PartialOrd, Eq)]
pub struct AssetTypeCode {
    pub val: ZeiAssetType
}

impl AssetTypeCode {
    pub fn new_from_base64(b64: &str) -> Result<Self> {
        let b = base64::decode(b64).c(d!());
        match b {
            Ok(mut bin) => {
                bin.resize(ASSET_TYPE_LENGTH, 0u8);
                let buf = <[u8; ASSET_TYPE_LENGTH]>::try_from(bin.as_slice()).c(d!())?;
                Ok(Self {
                    val: ZeiAssetType(buf),
                })
            }
            Err(e) => Err(eg!((format!(
                "Failed to deserialize base64 '{}': {}",
                b64, e
            )))),
        }
    }
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

impl AssetType {
    pub fn new_from_define_asset(da:&DefineAsset) -> Result<Self> {
        Ok(Self{
            properties: da.body.clone(),
            digest: [0;32],
            units: 0,
            confidential_units: Commitment([0;32])
        })
    }
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

impl DefineAsset {
    pub fn new_from_output(output: &tx_output) -> Result<Self> {
        if output.core.asset_type.is_confidential() {
            return Err(Box::from(d!("types 'confidential' not supported")));
        }
        let pubkey = output.core.public_key;
        Ok(Self {
            pubkey,
            body: Asset {
                asset_type_code: AssetTypeCode { val: output.core.asset_type.get_asset_type().unwrap() },
                issuer: pubkey,
                memo: "".to_string(),
                confidential_memo: None,
                policy: None
            },
            signature: None
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryTxOutPut {
    pub id: Input,
    pub record: Output,
    pub lien: Option<String>,
}

impl QueryTxOutPut {
    pub fn new_from_input_and_output(output: &tx_output, input: &tx_input) -> Result<Self> {
        let id = Input{
            txid: input.txid.clone(),
            n: input.n.clone()
        };

        let record = Output{ 
            core: output.core.clone(), 
            owner_memo: output.owner_memo.clone() 
        };

        Ok(Self{
            id,
            record,
            lien: None
        })

    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateCommitmentData {
    pub txns_in_block_hash: Vec<u8>,
    pub previous_state_commitment: Option<String>,
    pub txo_count: u64,
    pub staking: Option<String>,
}