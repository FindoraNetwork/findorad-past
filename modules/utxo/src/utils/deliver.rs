use std::collections::BTreeMap;

use super::{insert_owned_outputs_map, OwnedOutputOperation};
use crate::{Error, Result};
use abcf::bs3::MapStore;
use libfindora::{
    utxo::{Output, OutputId, Transaction, ValidateTransaction},
    Address,
};
use rand_chacha::ChaChaRng;
use zei::setup::PublicParams;

pub fn deliver_tx(
    params: &mut PublicParams,
    prng: &mut ChaChaRng,
    outputs_set: &mut impl MapStore<OutputId, Output>,
    tx: &Transaction,
) -> Result<BTreeMap<Address, Vec<OwnedOutputOperation>>> {
    let mut validate_tx = ValidateTransaction {
        inputs: Vec::new(),
        outputs: Vec::new(),
        proof: tx.proof.clone(),
    };

    let mut res = BTreeMap::new();

    for input in &tx.inputs {
        let record = outputs_set
            .remove(input)?
            .ok_or_else(|| Error::NoUnspentOutput(input.clone()))?;
        validate_tx
            .inputs
            .push(record.clone().to_blind_asset_record());

        insert_owned_outputs_map(
            &mut res,
            record.address,
            OwnedOutputOperation::Del(input.clone()),
        );
    }

    for output in &tx.outputs {
        validate_tx
            .outputs
            .push(output.clone().to_blind_asset_record());
    }

    validate_tx
        .verify(prng, params)
        .map_err(|e| Error::UtxoBalanceError(format!("Balance error: {:?}", e)))?;

    // let mut res = Vec::new();

    for i in 0..tx.outputs.len() {
        let output = &tx.outputs[i];
        let txid = &tx.txid;
        let n = i.try_into()?;

        let output_id = OutputId { txid: *txid, n };

        outputs_set.insert(output_id.clone(), output.clone())?;

        let owner = output.address.clone();

        insert_owned_outputs_map(
            &mut res,
            owner,
            OwnedOutputOperation::Add(output_id.clone()),
        );
    }

    Ok(res)
}
