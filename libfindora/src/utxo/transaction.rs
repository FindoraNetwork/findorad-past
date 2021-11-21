use std::convert::TryFrom;

use crate::transaction::Transaction;
use primitive_types::H512;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{
    AssetTypeAndAmountProof, BlindAssetRecord, OwnerMemo, XfrAmount, XfrAssetType,
};

use super::Address;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Input {
    pub txid: H512,
    pub n: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Output {
    pub amount: XfrAmount,
    pub asset: XfrAssetType,
    pub address: Address,
    pub owner_memo: Option<OwnerMemo>,
}

impl Output {
    pub fn to_blind_asset_record(self) -> BlindAssetRecord {
        BlindAssetRecord {
            amount: self.amount,
            asset_type: self.asset,
        }
    }

    pub fn from_blind_asset_record(
        ar: BlindAssetRecord,
        address: Address,
        owner_memo: Option<OwnerMemo>,
    ) -> Self {
        Self {
            amount: ar.amount,
            asset: ar.asset_type,
            address,
            owner_memo,
        }
    }
}

#[derive(Debug)]
pub struct UtxoTransacrion {
    pub txid: H512,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
}

impl Default for UtxoTransacrion {
    fn default() -> Self {
        UtxoTransacrion {
            txid: H512::zero(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
        }
    }
}

impl TryFrom<&Transaction> for UtxoTransacrion {
    type Error = abcf::Error;

    fn try_from(tx: &Transaction) -> Result<Self, Self::Error> {
        let mut inputs = Vec::new();

        for input in &tx.inputs {
            if input.txid == H512::zero() {
                inputs.push(Input {
                    txid: tx.txid.clone(),
                    n: input.n,
                })
            } else {
                inputs.push(Input {
                    txid: input.txid.clone(),

                    n: input.n,
                })
            }
        }

        let mut outputs = Vec::new();

        for output in &tx.outputs {
            outputs.push(output.core.clone());
        }

        Ok(Self {
            txid: tx.txid.clone(),
            inputs,
            outputs,
            proof: tx.proof.clone(),
        })
    }
}
