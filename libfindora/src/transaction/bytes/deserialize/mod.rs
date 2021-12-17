mod input;
mod output;
mod proof;
mod signature;

use crate::{transaction::Transaction, transaction_capnp::transaction, Result};
use primitive_types::H512;

pub fn from_root(root: transaction::Reader) -> Result<Transaction> {
    let txid = {
        let txid_slice = root.get_txid()?;
        let inner = txid_slice.try_into()?;
        H512(inner)
    };

    let mut inputs = Vec::new();

    for reader in root.get_inputs()?.iter() {
        inputs.push(input::from_input(reader)?);
    }

    let mut outputs = Vec::new();

    for reader in root.get_outputs()?.iter() {
        outputs.push(output::from_output(reader)?);
    }

    let proof = proof::from_proof(root.get_proof())?;

    let mut signatures = Vec::new();

    for reader in root.get_signature()?.iter() {
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
