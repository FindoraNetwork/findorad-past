use libfindora::Address;

use crate::types;

use rand_core::{CryptoRng, RngCore};
use serde::{Deserialize, Serialize};

use zei::xfr::asset_record::AssetRecordType;
use zei::xfr::structs::{AssetType, ASSET_TYPE_LENGTH};
use zei::xfr::{
    sig::{XfrKeyPair, XfrPublicKey},
    structs::{AssetRecord, AssetRecordTemplate},
};

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transfer {
    pub from: XfrKeyPair,

    /// Zei's public key, if you want to receive confidential transaction, this field must provide.
    pub public_key: Option<XfrPublicKey>,

    /// to
    pub address: Address,

    pub amount: u64,

    pub asset_type: AssetType,

    pub confidential_amount: bool,

    pub confidential_asset: bool,
}

#[derive(Default)]
pub struct TransferBuilder<'a> {
    from: &'a str,
    public_key: &'a str,
    address: &'a str,
    amount: u64,
    asset_type: &'a str,
    confidential_amount: bool,
    confidential_asset: bool,
}

impl<'a> TransferBuilder<'a> {
    pub fn from(mut self, f: &'a str) -> TransferBuilder {
        self.from = f;
        self
    }

    pub fn public_key(mut self, key: &'a str) -> TransferBuilder {
        self.public_key = key;
        self
    }

    pub fn address(mut self, addr: &'a str) -> TransferBuilder {
        self.address = addr;
        self
    }

    pub fn amount(mut self, a: u64) -> TransferBuilder<'a> {
        self.amount = a;
        self
    }

    pub fn asset_type(mut self, typ: &'a str) -> TransferBuilder {
        self.asset_type = typ;
        self
    }

    pub fn confidential_amount(mut self, b: bool) -> TransferBuilder<'a> {
        self.confidential_amount = b;
        self
    }

    pub fn confidential_asset(mut self, b: bool) -> TransferBuilder<'a> {
        self.confidential_asset = b;
        self
    }

    pub fn build(self) -> Result<Transfer> {
        let mut asset_type: [u8; ASSET_TYPE_LENGTH] = Default::default();
        let b_astyp = base64::decode_config(self.asset_type, base64::URL_SAFE)?;
        asset_type.copy_from_slice(&b_astyp);

        Ok(Transfer {
            from: types::SecretKey::from_base64(self.from)?.key.into_keypair(),
            public_key: Some(types::PublicKey::from_base64(self.public_key)?.key),
            address: Address::from(types::Address::from_base64(self.address)?.address),
            amount: self.amount,
            asset_type: AssetType(asset_type),
            confidential_amount: self.confidential_amount,
            confidential_asset: self.confidential_asset,
        })
    }
}

impl Transfer {
    pub fn builder() -> TransferBuilder<'static> {
        TransferBuilder::default()
    }

    pub fn to_output_asset_record<R: CryptoRng + RngCore>(
        &self,
        prng: &mut R,
    ) -> Result<AssetRecord> {
        let (pk, asset_record_type) = match self.public_key {
            None => {
                // If to ETH address, It only a placeholder to fit zei.
                // We need a zeilite.
                if !self.confidential_amount && !self.confidential_asset {
                    let asset_record_type = AssetRecordType::from_flags(false, false);
                    (self.from.get_pk(), asset_record_type)
                } else {
                    return Err(Error::MustBeNonConfidentialAssetAmount);
                }
            }
            Some(pk) => {
                let asset_record_type =
                    AssetRecordType::from_flags(self.confidential_amount, self.confidential_asset);
                (pk, asset_record_type)
            }
        };
        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            self.asset_type,
            asset_record_type,
            pk,
        );
        Ok(AssetRecord::from_template_no_identity_tracing(
            prng, &template,
        )?)
    }

    pub fn to_input_address(&self) -> Address {
        Address::from(self.from.get_pk())
    }

    pub fn to_keypair(&self) -> XfrKeyPair {
        self.from.clone()
    }

    pub fn is_confidential_amount(&self) -> bool {
        self.confidential_amount
    }

    pub fn is_confidential_asset(&self) -> bool {
        self.confidential_asset
    }
}

// pub async fn build_input_asset_record_and_id<R: CryptoRng + RngCore>(
//     prng: &mut R,
//     entries: Vec<TransferEntry>,
// ) -> Result<(
//     Vec<(XfrKeyPair, H512, u32, AssetRecord)>,
//     Vec<(AssetRecord, Address)>,
// )> {
//     let mut inputs = Vec::new();
//     let mut outputs = Vec::new();
//     let mut open_input = Vec::new();
//
//     let mut from_matix: BTreeMap<(XfrPublicKey, AssetType), u64> = BTreeMap::new();
//
//     let wallets: Vec<XfrKeyPair> = entries.iter().map(|e| e.from.clone()).collect();
//
//     let params = GetOwnedUtxoReq {
//         owners: wallets.iter().map(|w| w.get_pk().into()).collect(),
//     };
//
//     let provider = HttpGetProvider {};
//
//     let result = get_owned_outputs(provider, params)
//         .await
//         .map_err(|e| eg!(format!("{:?}", e)))?;
//
//     let from_outputs = result.data.c(d!())?.outputs;
//
//     for oai in from_outputs {
//         let keypair = &wallets[oai.0];
//
//         let output = oai.1.output;
//         let output_id = oai.1.output_id;
//
//         let core = BlindAssetRecord {
//             amount: output.amount.clone(),
//             asset_type: output.asset.clone(),
//         };
//
//         let open_asset_record = open_blind_asset_record(&core, &output.owner_memo, keypair)?;
//
//         open_input.push((keypair.clone(), output_id, open_asset_record.clone()));
//
//         let key = (
//             open_asset_record.get_pub_key().clone(),
//             open_asset_record.asset_type,
//         );
//
//         log::debug!("Open Asset Recore is: {:?}", open_asset_record);
//
//         if let Some(v) = from_matix.get_mut(&key) {
//             *v += open_asset_record.amount;
//         } else {
//             from_matix.insert(key, open_asset_record.amount);
//         }
//     }
//
//     // Build to_matix
//     let mut to_matix: BTreeMap<(XfrPublicKey, AssetType), u64> = BTreeMap::new();
//
//     for entry in entries {
//         let key = (entry.from.get_pk(), entry.asset_type);
//         if let Some(v) = to_matix.get_mut(&key) {
//             *v += entry.amount;
//         } else {
//             to_matix.insert(key, entry.amount);
//         }
//
//         let asset_record_type =
//             AssetRecordType::from_flags(entry.confidential_amount, entry.confidential_asset);
//
//         let template = AssetRecordTemplate::with_no_asset_tracing(
//             entry.amount,
//             entry.asset_type,
//             asset_record_type,
//             entry.to,
//         );
//
//         let asset_record = AssetRecord::from_template_no_identity_tracing(prng, &template)?;
//         outputs.push(asset_record);
//     }
//
//     //     println!("{:?}", from_matix);
//     //     println!("{:?}", to_matix);
//
//     for (k, v) in to_matix {
//         if let Some(amount) = from_matix.get(&k) {
//             if amount < &v {
//                 // No enough amount.
//                 return Err(eg!("target amount isn't enough"));
//             } else {
//                 let template = AssetRecordTemplate::with_no_asset_tracing(
//                     amount - v,
//                     k.1,
//                     AssetRecordType::NonConfidentialAmount_NonConfidentialAssetType,
//                     k.0,
//                 );
//
//                 let asset_record = AssetRecord::from_template_no_identity_tracing(prng, &template)?;
//                 outputs.push(asset_record);
//
//                 for (keypair, output_id, open_asset_record) in &open_input {
//                     if open_asset_record.asset_type == k.1
//                         && open_asset_record.blind_asset_record.public_key == k.0
//                     {
//                         let asset_record = AssetRecord::from_open_asset_record_no_asset_tracing(
//                             open_asset_record.clone(),
//                         );
//
//                         inputs.push((
//                             keypair.clone(),
//                             output_id.txid.clone(),
//                             output_id.n,
//                             asset_record,
//                         ));
//                     }
//                 }
//             }
//         } else {
//             return Err(eg!("no this asset for this user"));
//         }
//     }
//
//     Ok((inputs, outputs))
// }
