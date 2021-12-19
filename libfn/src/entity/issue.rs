use crate::Result;
use libfindora::asset::{Amount, AssetType};
use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};
use zei::xfr::{
    asset_record::AssetRecordType,
    sig::XfrKeyPair,
    structs::{AssetRecord, AssetRecordTemplate},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Issue {
    pub amount: Amount,
    pub asset_type: AssetType,
    pub confidential_amount: bool,
    pub keypair: XfrKeyPair,
}

impl Issue {
    pub fn to_output_asset_record<R: CryptoRng + RngCore>(
        &self,
        prng: &mut R,
    ) -> Result<AssetRecord> {
        let asset_record_type = AssetRecordType::from_flags(self.confidential_amount, false);

        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            self.asset_type,
            asset_record_type,
            self.keypair.get_pk(),
        );

        Ok(AssetRecord::from_template_no_identity_tracing(
            prng, &template,
        )?)
    }

    pub fn to_keypair(&self) -> XfrKeyPair {
        self.keypair.clone()
    }

    pub fn is_confidential(&self) -> bool {
        self.confidential_amount
    }
}
