mod input;
mod output;
mod proof;
mod signature;

use crate::{
    error::{convert_capnp_error, convert_try_slice_error},
    transaction::Transaction,
    transaction_capnp::transaction,
};
use primitive_types::H512;

pub fn from_root(root: transaction::Reader) -> abcf::Result<Transaction> {
    let txid = {
        let txid_slice = root.get_txid().map_err(convert_capnp_error)?;
        let inner = txid_slice.try_into().map_err(convert_try_slice_error)?;
        H512(inner)
    };

    let mut inputs = Vec::new();

    for reader in root.get_inputs().map_err(convert_capnp_error)?.iter() {
        inputs.push(input::from_input(reader)?);
    }

    let mut outputs = Vec::new();

    for reader in root.get_outputs().map_err(convert_capnp_error)?.iter() {
        outputs.push(output::from_output(reader)?);
    }

    let proof = proof::from_proof(root.get_proof())?;

    let mut signatures = Vec::new();

    for reader in root.get_signature().map_err(convert_capnp_error)?.iter() {
        signatures.push(signature::from_signature(reader)?);
    }

    Ok(Transaction {
        txid,
        inputs,
        outputs,
        signatures,
        proof,
    })
}
