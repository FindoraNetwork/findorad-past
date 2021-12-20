mod evm;
mod input;
mod output;
mod proof;
mod signature;

use crate::{transaction::Transaction, transaction_capnp::transaction, Result};

pub fn build_transaction(tx: &Transaction, builder: transaction::Builder) -> Result<()> {
    let mut builder = builder;

    builder.set_txid(tx.txid.0.as_ref());

    let inputs_len = tx.inputs.len().try_into()?;
    let mut inputs_builder = builder.reborrow().init_inputs(inputs_len);
    for index in 0..tx.inputs.len() {
        let builder = inputs_builder.reborrow().get(index.try_into()?);
        let input = &tx.inputs[index];
        input::build_input(input, builder)?;
    }

    let outputs_len = tx.outputs.len().try_into()?;
    let mut output_builder = builder.reborrow().init_outputs(outputs_len);
    for index in 0..tx.outputs.len() {
        let builder = output_builder.reborrow().get(index.try_into()?);
        let output = &tx.outputs[index];
        output::build_output(output, builder)?;
    }

    let proof_builder = builder.reborrow().init_proof();
    proof::build_proof(&tx.proof, proof_builder)?;

    let signature_len = tx.signatures.len().try_into()?;
    let mut siganture_builder = builder.init_signature(signature_len);
    for index in 0..tx.signatures.len() {
        let builder = siganture_builder.reborrow().get(index.try_into()?);
        let signature = &tx.signatures[index];
        signature::build_signature(signature, builder)?;
    }

    Ok(())
}
