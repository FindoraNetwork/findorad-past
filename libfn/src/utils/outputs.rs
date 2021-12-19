use libfindora::utxo::Output;
use zei::xfr::{
    asset_record::open_blind_asset_record,
    sig::XfrKeyPair,
    structs::{AssetRecord, BlindAssetRecord},
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
