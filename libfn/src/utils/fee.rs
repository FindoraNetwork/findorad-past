use fm_fee::FRA_FEE_AMOUNT;
use libfindora::asset::FRA;
use rand_core::{CryptoRng, RngCore};
use zei::xfr::{
    asset_record::AssetRecordType,
    structs::{AssetRecord, AssetRecordTemplate},
};

use crate::Result;

pub fn build_fee<R: RngCore + CryptoRng>(prng: &mut R) -> Result<AssetRecord> {
    let asset_record_type = AssetRecordType::from_flags(false, false);

    let template = AssetRecordTemplate::with_no_asset_tracing(
        FRA_FEE_AMOUNT,
        FRA.bare_asset_type,
        asset_record_type,
        Default::default(),
    );

    Ok(AssetRecord::from_template_no_identity_tracing(
        prng, &template,
    )?)
}
