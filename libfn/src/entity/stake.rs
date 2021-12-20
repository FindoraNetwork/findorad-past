use libfindora::{
    asset::{Amount, FRA},
    staking::{self, TendermintAddress, ValidatorPublicKey},
};
use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use zei::xfr::{
    asset_record::AssetRecordType,
    sig::XfrKeyPair,
    structs::{AssetRecord, AssetRecordTemplate},
};

use crate::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct StakeMemo {
    pub name: String,
    pub website: String,
    pub desc: String,
    // rate * 6,
    pub rate: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stake {
    pub amount: Amount,
    pub validator: TendermintAddress,
    pub public_key: ValidatorPublicKey,
    pub keypair: XfrKeyPair,
    pub memo: StakeMemo,
}

impl Stake {
    pub fn to_output<R: CryptoRng + RngCore>(&self, prng: &mut R) -> Result<AssetRecord> {
        let asset_record_type = AssetRecordType::from_flags(false, false);

        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            FRA.bare_asset_type,
            asset_record_type,
            self.keypair.get_pk(),
        );

        Ok(AssetRecord::from_template_no_identity_tracing(
            prng, &template,
        )?)
    }

    pub fn to_operation(&self) -> Result<staking::Delegate> {
        let address = self.validator.clone();

        Ok(staking::Delegate {
            address,
            memo: Some(serde_json::to_vec(&self.memo)?),
            validator: Some(self.public_key.clone()),
        })
    }

    pub fn to_keypair(&self) -> XfrKeyPair {
        self.keypair.clone()
    }
}
