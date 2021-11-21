use abcf_sdk::providers::HttpGetProvider;
use fm_utxo::utxo_module_rpc::get_owned_outputs;
use libfindora::utxo::{Address, GetOwnedUtxoReq};
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use zei::xfr::asset_record::{open_blind_asset_record, AssetRecordType};
use zei::xfr::structs::AssetType;
use zei::xfr::{
    sig::{XfrKeyPair, XfrPublicKey},
    structs::{AssetRecord, AssetRecordTemplate},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferEntry {
    pub from: XfrKeyPair,
    pub to: Address,
    pub amount: u64,
    pub asset_type: AssetType,
    pub confidential_amount: bool,
    pub confidential_asset: bool,
}

impl TransferEntry {
    pub fn to_output_asset_record<R: CryptoRng + RngCore>(
        self,
        prng: &mut R,
    ) -> Result<AssetRecord> {
        let asset_record_type = AssetRecordType::from_flags(self.confidential_amount, false);

        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            self.asset_type,
            asset_record_type,
            self.to,
        );

        AssetRecord::from_template_no_identity_tracing(prng, &template)
    }
}

pub async fn build_input_asset_record_and_id<R: CryptoRng + RngCore>(
    prng: &mut R,
    entries: Vec<TransferEntry>,
) -> Result<(
    Vec<(XfrKeyPair, Vec<u8>, u32, AssetRecord)>,
    Vec<AssetRecord>,
)> {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut open_input = Vec::new();

    let mut from_matix: BTreeMap<(XfrPublicKey, AssetType), u64> = BTreeMap::new();

    let wallets: Vec<XfrKeyPair> = entries.iter().map(|e| e.from.clone()).collect();

    let params = GetOwnedUtxoReq {
        owners: wallets.iter().map(|w| w.get_pk()).collect(),
    };

    let provider = HttpGetProvider {};

    let result = get_owned_outputs(provider, params)
        .await
        .map_err(|e| eg!(format!("{:?}", e)))?;

    let from_outputs = result.data.c(d!())?.outputs;

    for oai in from_outputs {
        let keypair = &wallets[oai.0];

        let output = oai.1.output;
        let output_id = oai.1.output_id;

        let open_asset_record = open_blind_asset_record(&output.core, &output.owner_memo, keypair)?;

        open_input.push((keypair.clone(), output_id, open_asset_record.clone()));

        let key = (
            open_asset_record.get_pub_key().clone(),
            open_asset_record.asset_type,
        );

        log::debug!("Open Asset Recore is: {:?}", open_asset_record);

        if let Some(v) = from_matix.get_mut(&key) {
            *v += open_asset_record.amount;
        } else {
            from_matix.insert(key, open_asset_record.amount);
        }
    }

    // Build to_matix
    let mut to_matix: BTreeMap<(XfrPublicKey, AssetType), u64> = BTreeMap::new();

    for entry in entries {
        let key = (entry.from.get_pk(), entry.asset_type);
        if let Some(v) = to_matix.get_mut(&key) {
            *v += entry.amount;
        } else {
            to_matix.insert(key, entry.amount);
        }

        let asset_record_type =
            AssetRecordType::from_flags(entry.confidential_amount, entry.confidential_asset);

        let template = AssetRecordTemplate::with_no_asset_tracing(
            entry.amount,
            entry.asset_type,
            asset_record_type,
            entry.to,
        );

        let asset_record = AssetRecord::from_template_no_identity_tracing(prng, &template)?;
        outputs.push(asset_record);
    }

    //     println!("{:?}", from_matix);
    //     println!("{:?}", to_matix);

    for (k, v) in to_matix {
        if let Some(amount) = from_matix.get(&k) {
            if amount < &v {
                // No enough amount.
                return Err(eg!("target amount isn't enough"));
            } else {
                let template = AssetRecordTemplate::with_no_asset_tracing(
                    amount - v,
                    k.1,
                    AssetRecordType::NonConfidentialAmount_NonConfidentialAssetType,
                    k.0,
                );

                let asset_record = AssetRecord::from_template_no_identity_tracing(prng, &template)?;
                outputs.push(asset_record);

                for (keypair, output_id, open_asset_record) in &open_input {
                    if open_asset_record.asset_type == k.1
                        && open_asset_record.blind_asset_record.public_key == k.0
                    {
                        let asset_record = AssetRecord::from_open_asset_record_no_asset_tracing(
                            open_asset_record.clone(),
                        );

                        inputs.push((
                            keypair.clone(),
                            output_id.txid.clone(),
                            output_id.n,
                            asset_record,
                        ));
                    }
                }
            }
        } else {
            return Err(eg!("no this asset for this user"));
        }
    }

    Ok((inputs, outputs))
}
