use primitive_types::H512;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{BlindAssetRecord, OwnerMemo, XfrAmount, XfrAssetType};

use crate::Address;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct OutputId {
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
