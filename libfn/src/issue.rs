use libfindora::Amount;
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::xfr::asset_record::AssetRecordType;
use zei::xfr::sig::XfrKeyPair;
use zei::xfr::structs::AssetType;
use zei::xfr::structs::{AssetRecord, AssetRecordTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueEntry {
    pub amount: Amount,
    pub asset_type: AssetType,
    pub confidential_amount: bool,
    pub keypair: XfrKeyPair,
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

    pub fn to_keypair(&self) -> XfrKeyPair {
        self.keypair.clone()
    }

    pub fn to_input_amount(&self) -> Result<(AssetType, Amount)> {
        Ok((self.asset_type, self.amount))
    }

    pub fn to_output_amount(&self) -> Result<(AssetType, Amount)> {
        Ok((self.asset_type, 0))
    }
}
