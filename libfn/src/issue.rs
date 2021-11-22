use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::xfr::asset_record::AssetRecordType;
use zei::xfr::structs::AssetType;
use zei::xfr::{
    sig::XfrKeyPair,
    structs::{AssetRecord, AssetRecordTemplate},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueEntry {
    pub keypair: XfrKeyPair,
    pub amount: u64,
    pub asset_type: AssetType,
    pub confidential_amount: bool,
}

impl IssueEntry {
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

        AssetRecord::from_template_no_identity_tracing(prng, &template)
    }
}
