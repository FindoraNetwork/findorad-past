use abcf::tm_protos::crypto;
use abcf_sdk::providers::HttpGetProvider;
use fm_utxo::utxo_module_rpc::get_owned_outputs;
use libfindora::staking::{Delegate, TendermintAddress};
use libfindora::utxo::GetOwnedUtxoReq;
use libfindora::FRA_ASSET_TYPE;
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use zei::xfr::asset_record::{open_blind_asset_record, AssetRecordType};
use zei::xfr::sig::XfrPublicKey;
use zei::xfr::{
    sig::XfrKeyPair,
    structs::{AssetRecord, AssetRecordTemplate},
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TdPubkeyType {
    PubKeyEd25519,
    PubKeySecp256k1,
}

impl TdPubkeyType {
    pub fn from_str(s: &str) -> Result<Self> {
        return match s {
            "PubKeyEd25519" => Ok(Self::PubKeyEd25519),
            "PubKeySecp256k1" => Ok(Self::PubKeySecp256k1),
            _ => Err(eg!("not match")),
        };
    }

    pub fn to_sum(&self, bytes: Vec<u8>) -> Option<crypto::public_key::Sum> {
        return match self {
            TdPubkeyType::PubKeyEd25519 => Some(crypto::public_key::Sum::Ed25519(bytes)),
            TdPubkeyType::PubKeySecp256k1 => Some(crypto::public_key::Sum::Secp256k1(bytes)),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelegationEntry {
    pub keypair: XfrKeyPair,
    pub amount: u64,
    pub validator_address: TendermintAddress,
    pub validator_ty_pubkey: Option<(TdPubkeyType, Vec<u8>)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UnDelegationEntry {
    pub keypair: XfrKeyPair,
    pub amount: u64,
    pub validator_address: TendermintAddress,
}

impl DelegationEntry {
    pub fn to_output_asset_record<R: CryptoRng + RngCore>(
        &self,
        prng: &mut R,
    ) -> Result<(AssetRecord, Delegate)> {
        let asset_record_type = AssetRecordType::from_flags(false, false);

        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            FRA_ASSET_TYPE,
            asset_record_type,
            self.keypair.get_pk(),
        );

        let pubkey = if let Some((ty, bytes)) = &self.validator_ty_pubkey {
            let sum = ty.to_sum(bytes.clone());
            Some(crypto::PublicKey { sum })
        } else {
            None
        };

        let delegate = Delegate {
            address: self.validator_address.clone(),
            validator: pubkey,
            memo: None,
        };

        let ar = AssetRecord::from_template_no_identity_tracing(prng, &template)?;
        Ok((ar, delegate))
    }
}

impl UnDelegationEntry {
    pub fn to_output_asset_record<R: CryptoRng + RngCore>(
        &self,
        prng: &mut R,
    ) -> Result<AssetRecord> {
        let asset_record_type = AssetRecordType::from_flags(false, false);

        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            FRA_ASSET_TYPE,
            asset_record_type,
            self.keypair.get_pk(),
        );

        AssetRecord::from_template_no_identity_tracing(prng, &template)
    }
}

pub async fn delegation_build_input_asset_record_and_id<R: CryptoRng + RngCore>(
    prng: &mut R,
    entries: Vec<DelegationEntry>,
) -> Result<(
    Vec<(XfrKeyPair, Vec<u8>, u32, AssetRecord)>,
    Vec<AssetRecord>,
)> {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut open_input = Vec::new();

    let mut from_matix: BTreeMap<XfrPublicKey, u64> = BTreeMap::new();

    // xfr utxo available for each pubkey
    let wallets: Vec<XfrKeyPair> = entries.iter().map(|e| e.keypair.clone()).collect();

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

        // only use fra asset type
        if open_asset_record.asset_type == FRA_ASSET_TYPE {
            open_input.push((keypair.clone(), output_id, open_asset_record.clone()));

            let key = open_asset_record.get_pub_key().clone();

            log::debug!("Open Asset Recore is: {:?}", open_asset_record);

            if let Some(v) = from_matix.get_mut(&key) {
                *v += open_asset_record.amount;
            } else {
                from_matix.insert(key, open_asset_record.amount);
            }
        }
    }

    for entry in entries {
        let pk = entry.keypair.get_pk();
        let pk_base64 = base64::encode(&pk.as_bytes());
        if let Some(a) = from_matix.get(&pk) {
            if a < &entry.amount {
                return Err(eg!(format!("target amount isn't enough:{}", pk_base64)));
            } else {
                let template = AssetRecordTemplate::with_no_asset_tracing(
                    a - entry.amount,
                    FRA_ASSET_TYPE,
                    AssetRecordType::NonConfidentialAmount_NonConfidentialAssetType,
                    pk.clone(),
                );
                let asset_record = AssetRecord::from_template_no_identity_tracing(prng, &template)?;
                outputs.push(asset_record);

                for (keypair, output_id, open_asset_record) in &open_input {
                    if open_asset_record.blind_asset_record.public_key == pk {
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
            return Err(eg!(format!("no xfr asset for this user:{}", pk_base64)));
        }
    }

    Ok((inputs, outputs))
}
