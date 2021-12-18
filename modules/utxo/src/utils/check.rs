use crate::{Error, Result};
use abcf::bs3::MapStore;
use libfindora::utxo::{Output, OutputId, Transaction, ValidateTransaction};
use rand_chacha::ChaChaRng;
use zei::setup::PublicParams;

pub fn check_tx(
    params: &mut PublicParams,
    prng: &mut ChaChaRng,
    outputs_set: &impl MapStore<OutputId, Output>,
    tx: &Transaction,
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
