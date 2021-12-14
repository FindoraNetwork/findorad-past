use libfindora::transaction::{Input, InputOperation, Output, OutputOperation, Transaction};
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::xfr::lib::gen_xfr_body;
use zei::xfr::sig::XfrKeyPair;
use zei::xfr::structs::AssetRecord;

mod issue;
pub use issue::IssueEntry;
use libfindora::staking::Undelegate;

mod stake;
mod transfer;
mod wallet;
pub use stake::{DelegationEntry, TdPubkeyType, UnDelegationEntry};

pub use wallet::AccountEntry;

pub use transfer::TransferEntry;

use stake::delegation_build_input_asset_record_and_id;
use transfer::transfer_build_input_asset_record_and_id;

#[derive(Serialize, Deserialize, Debug)]
pub enum Entry {
    Issue(IssueEntry),
    Transfer(TransferEntry),
    Delegation(DelegationEntry),
    UnDelegation(UnDelegationEntry),
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
    let mut delegation_entry = Vec::new();

    let mut index = 0;

    for entry in entries {
        match entry {
            Entry::Issue(e) => {
                keypairs.push(e.keypair.clone());
                let output = e.to_output_asset_record(prng)?;
                input_ids.push((Vec::new(), index, InputOperation::IssueAsset));
                output_ids.push(OutputOperation::IssueAsset);
                inputs.push(output.clone());
                outputs.push(output);
            }
            Entry::Transfer(e) => {
                transfer_entry.push(e);
            }
            Entry::Delegation(e) => {
                delegation_entry.push(e.clone());
                // add delegation output
                let output = e.to_output_asset_record(prng)?;
                output_ids.push(OutputOperation::Delegate(output.1));
                outputs.push(output.0);
            }
            Entry::UnDelegation(e) => {
                keypairs.push(e.keypair.clone());
                let output = e.to_output_asset_record(prng)?;
                input_ids.push((Vec::new(), index, InputOperation::Undelegate));
                output_ids.push(OutputOperation::Undelegate(Undelegate {
                    address: e.validator_address.clone(),
                }));
                inputs.push(output.clone());
                outputs.push(output);
            }
        };
        index += 1;
    }

    let mut ios: (
        Vec<(XfrKeyPair, Vec<u8>, u32, AssetRecord)>,
        Vec<AssetRecord>,
    ) = (Vec::new(), Vec::new());

    {
        let mut transfer_ios =
            transfer_build_input_asset_record_and_id(prng, transfer_entry).await?;
        ios.0.append(&mut transfer_ios.0);
        ios.1.append(&mut transfer_ios.1);

        let mut delegation_ios =
            delegation_build_input_asset_record_and_id(prng, delegation_entry).await?;
        ios.0.append(&mut delegation_ios.0);
        ios.1.append(&mut delegation_ios.1);
    }

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
