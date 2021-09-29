use libfindora::transaction::{Input, InputOperation, Output, OutputOperation, Transaction};
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::xfr::lib::gen_xfr_body;

mod issue;
pub use issue::IssueEntry;

mod transfer;
pub use transfer::TransferEntry;

use self::transfer::build_input_asset_record_and_id;

mod utils;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Issue(IssueEntry),
    Transfer(TransferEntry),
}

pub async fn build_transaction<R: CryptoRng + RngCore>(
    prng: &mut R,
    entries: Vec<Entry>,
) -> Result<Transaction> {
    let mut input_ids = Vec::new();
    let mut inputs = Vec::new();
    let mut output_ids = Vec::new();
    let mut outputs = Vec::new();
    let mut keypairs = Vec::new();

    let mut transfer_entry = Vec::new();

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
            Entry::Transfer(e) => {
                transfer_entry.push(e);
            }
        };
    }

    let ios = build_input_asset_record_and_id(prng, transfer_entry).await?;

    for input in ios.0 {
        keypairs.push(input.0);
        input_ids.push((input.1, input.2, InputOperation::TransferAsset));
        inputs.push(input.3);
    }

    for output in ios.1 {
        output_ids.push(OutputOperation::TransferAsset);
        outputs.push(output);
    }

    log::debug!("Inputs is : {:?}", inputs);
    log::debug!("Outputs is : {:?}", outputs);

    let zei_body = gen_xfr_body(prng, &inputs, &outputs)?;

    let mut tx_inputs = Vec::new();
    let mut tx_outputs = Vec::new();

    for iii in input_ids {
        tx_inputs.push(Input {
            txid: iii.0,
            n: iii.1,
            operation: iii.2,
        });
    }

    for i in 0..zei_body.outputs.len() {
        let operation = &output_ids[i];
        let owner_memo = &zei_body.owners_memos[i];
        let core = &zei_body.outputs[i];
        tx_outputs.push(Output {
            core: core.clone(),
            operation: operation.clone(),
            owner_memo: owner_memo.clone(),
        });
    }

    let mut tx = Transaction {
        txid: Vec::new(),
        inputs: tx_inputs,
        outputs: tx_outputs,
        proof: zei_body.proofs.asset_type_and_amount_proof,
        signatures: Vec::new(),
    };

    log::debug!("Result tx is: {:?}", tx);

    tx.signature(keypairs)?;

    Ok(tx)
}
