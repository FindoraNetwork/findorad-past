use std::collections::BTreeMap;

use crate::{Error, Result};
use libfindora::{utxo::OutputId, Address};

pub enum OwnedOutputOperation {
    Add(OutputId),
    Del(OutputId),
}

fn get_index(array: &[OutputId], target: &OutputId) -> Option<usize> {
    array
        .iter()
        .position(|x| x.txid == target.txid && x.n == target.n)
}

pub fn insert_by_operation(
    target: &mut Vec<OutputId>,
    ops: Vec<OwnedOutputOperation>,
) -> Result<()> {
    for op in ops {
        match op {
            OwnedOutputOperation::Add(v) => {
                // if v already in target, failed.
                if get_index(target, &v).is_none() {
                    target.push(v);
                } else {
                    return Err(Error::DuplicateOutput(v));
                }
            }
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

pub fn insert_owned_outputs_map(
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
