use std::collections::BTreeMap;

use crate::{Error, Result};
use abcf::bs3::MapStore;
use libfindora::{
    utxo::{Output, OutputId, UtxoTransaction, ValidateTransaction},
    Address,
};
use rand_chacha::ChaChaRng;
use zei::setup::PublicParams;

pub fn check_tx(
    params: &mut PublicParams,
    prng: &mut ChaChaRng,
    outputs_set: &impl MapStore<OutputId, Output>,
    tx: &UtxoTransaction,
) -> Result<()> {
    let mut validate_tx = ValidateTransaction {
        inputs: Vec::new(),
        outputs: Vec::new(),
        proof: tx.proof.clone(),
    };

    for input in &tx.inputs {
        let record = outputs_set
            .get(input)?
            .ok_or_else(|| Error::NoUnspentOutput(input.clone()))?;
        validate_tx
            .inputs
            .push(record.clone().to_blind_asset_record());
    }

    for output in &tx.outputs {
        validate_tx
            .outputs
            .push(output.clone().to_blind_asset_record());
    }

    validate_tx
        .verify(prng, params)
        .map_err(|e| Error::UtxoBalanceError(format!("Balance error: {:?}", e)))?;

    Ok(())
}

pub enum OwnedOutputOperation {
    Add(OutputId),
    Del(OutputId),
}

fn get_index(array: &Vec<OutputId>, target: &OutputId) -> Option<usize> {
    array.iter().position(|x| x.txid == target.txid && x.n == target.n)
}

pub fn insert_by_operation(target: &mut Vec<OutputId>, ops: Vec<OwnedOutputOperation>) -> Result<()> {
    for op in ops {
        match op {
            OwnedOutputOperation::Add(v) => {
                // if v already in target, failed.
                if let None = get_index(target, &v) {
                    target.push(v);
                } else {
                    return Err(Error::DuplicateOutput(v))
                }
            },
            OwnedOutputOperation::Del(v) => {
                if let Some(index) = get_index(target, &v) {
                    target.remove(index);
                } else {
                    return Err(Error::MissingOutput(v));
                }
            }
        }
    }

    Ok(())
}

fn insert_owned_outputs_map(
    map: &mut BTreeMap<Address, Vec<OwnedOutputOperation>>,
    owner: Address,
    output_id: OwnedOutputOperation,
) {
    if let Some(v) = map.get_mut(&owner) {
        v.push(output_id);
    } else {
        let v = vec![output_id];
        map.insert(owner, v);
    }
}

pub fn deliver_tx(
    params: &mut PublicParams,
    prng: &mut ChaChaRng,
    outputs_set: &mut impl MapStore<OutputId, Output>,
    tx: &UtxoTransaction,
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
