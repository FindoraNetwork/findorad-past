use libfindora::transaction::{Input, InputOperation, Output, OutputOperation, Transaction};
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::xfr::asset_record::AssetRecordType;
use zei::xfr::lib::gen_xfr_body;
use zei::xfr::structs::{AssetType, XfrAssetType};
use zei::xfr::{
    sig::{XfrKeyPair, XfrPublicKey},
    structs::{AssetRecord, AssetRecordTemplate},
};
use zei::serialization::ZeiFromToBytes;
use bech32::{Variant, ToBase32};
use crate::config::Config;

#[derive(Serialize, Deserialize, Debug)]
pub struct IssueEntry {
    pub keypair: XfrKeyPair,
    pub amount: u64,
    pub asset_type: AssetType,
    pub confidential_amount: bool,
}

impl IssueEntry {
    pub fn to_output_asset_record<R: CryptoRng + RngCore>(
        self,
        prng: &mut R,
    ) -> Result<AssetRecord> {
        let asset_record_type = if self.confidential_amount {
            AssetRecordType::ConfidentialAmount_NonConfidentialAssetType
        } else {
            AssetRecordType::NonConfidentialAmount_NonConfidentialAssetType
        };

        let template = AssetRecordTemplate::with_no_asset_tracing(
            self.amount,
            self.asset_type,
            asset_record_type,
            self.keypair.get_pk(),
        );

        AssetRecord::from_template_no_identity_tracing(prng, &template)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferEntry {
    pub from: XfrKeyPair,
    pub to: XfrPublicKey,
    pub amount: u64,
    pub asset_type: XfrAssetType,
    pub confidential_amount: bool,
    pub confidential_asset: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Issue(IssueEntry),
    Transfer(TransferEntry),
}

pub fn build_transaction<R: CryptoRng + RngCore>(
    prng: &mut R,
    entries: Vec<Entry>,
) -> Result<Transaction> {
    let mut input_ids = Vec::new();
    let mut inputs = Vec::new();
    let mut output_ids = Vec::new();
    let mut outputs = Vec::new();
    let mut keypairs = Vec::new();

    for entry in entries {
        match entry {
            Entry::Issue(e) => {
                keypairs.push(e.keypair.clone());
                let output = e.to_output_asset_record(prng)?;
                input_ids.push((Vec::new(), 0, InputOperation::IssueAsset));
                output_ids.push(OutputOperation::IssueAsset);
                inputs.push(output.clone());
                outputs.push(output);
            }
            Entry::Transfer(_e) => {}
        };
    }

    let mut zei_body = gen_xfr_body(prng, &inputs, &outputs)?;

    let mut tx_inputs = Vec::new();
    let mut tx_outputs = Vec::new();

    for iii in input_ids {
        tx_inputs.push(Input {
            txid: iii.0,
            n: iii.1,
            operation: iii.2,
        });
    }

    for _ in 0..zei_body.outputs.len() {
        let operation = output_ids.pop().c(d!())?;
        let owner_memo = zei_body.owners_memos.pop().c(d!())?;
        let core = zei_body.outputs.pop().c(d!())?;
        tx_outputs.push(Output {
            core,
            operation,
            owner_memo,
        });
    }

    let mut tx = Transaction {
        txid: Vec::new(),
        inputs: tx_inputs,
        outputs: tx_outputs,
        proof: zei_body.proofs.asset_type_and_amount_proof,
        signatures: Vec::new(),
    };

    tx.signature(keypairs)?;

    Ok(tx)
}

pub struct BipPath {
    pub coin: u32,
    pub account: u32,
    pub change: u32,
    pub address: u32,
}

impl BipPath {
    pub fn new_fra() -> Self{
        Self{
            coin: 917,
            account: 0,
            change: 0,
            address: 0
        }
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub public_key:String,
    pub secret_key:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base64KeyPair {
    pub key_pair: KeyPair,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bech32KeyPair {
    pub key_pair: KeyPair,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountEntry{
    pub mnemonic: String,
    pub base64: Base64KeyPair,
    pub bech32: Bech32KeyPair,
}

impl AccountEntry {

    pub fn form_keypair(kp: XfrKeyPair, phrase: String) -> Result<Self>{
        let base64_pub_key = base64::encode(&kp.get_pk().zei_to_bytes());
        let base64_sec_key = base64::encode(&kp.get_sk().zei_to_bytes());

        let bech32_pub_key = bech32::encode("fra",
                                            &kp.get_pk().zei_to_bytes().to_base32(),
                                            Variant::Bech32).c(d!())?;
        let bech32_sec_key = bech32::encode("fra",
                                            &kp.get_sk().zei_to_bytes().to_base32(),
                                            Variant::Bech32).c(d!())?;

        Ok(Self{
            mnemonic: phrase,
            base64: Base64KeyPair {
                key_pair: KeyPair {
                    public_key: base64_pub_key,
                    secret_key: base64_sec_key
                }
            },
            bech32: Bech32KeyPair {
                key_pair: KeyPair {
                    public_key: bech32_pub_key,
                    secret_key: bech32_sec_key
                }
            }
        })
    }

    pub fn save(aes: &mut Vec<AccountEntry>, config: &Config, is_cover: bool) -> Result<()>{
        let mut path = config.node.home.clone();
        path.push("_account");

        let bytes = if !is_cover {
            let mut vec = AccountEntry::read(config).c(d!())?;
            vec.append(aes);
            serde_json::to_vec(&vec).c(d!())?
        } else {
            serde_json::to_vec(aes).c(d!())?
        };

        std::fs::write(path.as_path(),bytes).c(d!())?;

        Ok(())
    }

    pub fn read(config: &Config) -> Result<Vec<AccountEntry>> {

        let mut path = config.node.home.clone();
        path.push("_account");

        if !path.as_path().exists() {
            std::fs::File::create(path.as_path()).c(d!())?;
            let vec:Vec<AccountEntry> = Vec::new();
            return Ok(vec);
        }

        let data = std::fs::read(path).c(d!())?;
        let vec = serde_json::from_slice::<Vec<AccountEntry>>(data.as_slice())
            .c(d!())?;

        Ok(vec)
    }

    pub fn delete(index: usize, config: &Config) -> Result<AccountEntry> {
        let mut vec = AccountEntry::read(config).c(d!())?;
        let ae = vec.remove(index);

        AccountEntry::save(&mut vec,config, true).c(d!())?;
        Ok(ae)
    }
}