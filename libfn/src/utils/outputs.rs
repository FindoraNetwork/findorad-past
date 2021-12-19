use libfindora::{
    asset::{Amount, AssetType},
    utxo::Output,
};
use rand_core::{CryptoRng, RngCore};
use zei::xfr::{
    asset_record::{open_blind_asset_record, AssetRecordType},
    sig::{XfrKeyPair, XfrPublicKey},
    structs::{AssetRecord, AssetRecordTemplate, BlindAssetRecord},
};

use crate::Result;

pub fn open_outputs(outputs: Vec<Output>, keypair: &XfrKeyPair) -> Result<Vec<AssetRecord>> {
    let mut ars = Vec::new();

    for output in outputs {
        let record = BlindAssetRecord {
            asset_type: output.asset,
            amount: output.amount,
        };

        let oar = open_blind_asset_record(&record, &output.owner_memo, keypair)?;

        let ar = AssetRecord::from_open_asset_record_no_asset_tracing(oar);

        ars.push(ar);
    }

    Ok(ars)
}

pub fn build_output<R: RngCore + CryptoRng>(
    prng: &mut R,
    asset: AssetType,
    amount: Amount,
    confidential_asset: bool,
    confidential_amount: bool,
    public_key: XfrPublicKey,
) -> Result<AssetRecord> {
    let asset_record_type = AssetRecordType::from_flags(confidential_amount, confidential_asset);

    let template =
        AssetRecordTemplate::with_no_asset_tracing(amount, asset, asset_record_type, public_key);

    Ok(AssetRecord::from_template_no_identity_tracing(
        prng, &template,
    )?)
}
