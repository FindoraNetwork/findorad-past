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

    for entry in entries {
        match entry {
            Entry::Issue(e) => {
                let output = e.to_output_asset_record(prng)?;
                input_ids.push((Vec::new(), 0, InputOperation::IssueAsset));
                output_ids.push(OutputOperation::IssueAsset);
                inputs.push(output.clone());
                outputs.push(output);
            }
            Entry::Transfer(_e) => {}
        };
    }

    let mut zei_body = gen_xfr_body(&mut prng, &inputs, &outputs)?;

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

    Ok(Transaction {
        txid: Vec::new(),
        inputs: tx_inputs,
        outputs: tx_outputs,
        proof: zei_body.proofs.asset_type_and_amount_proof,
        signatures: Vec::new(),
    })
}
