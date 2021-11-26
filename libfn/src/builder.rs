use libfindora::{
    transaction::{InputOperation, OutputOperation},
    utxo::Address,
};
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use zei::xfr::structs::AssetRecord;

use crate::Entry;

pub struct Input {
    pub record: AssetRecord,
    pub operation: InputOperation,
}

pub struct Output {
    pub record: AssetRecord,
    pub operation: OutputOperation,
    pub address: Address,
}

pub struct Builder {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
}

impl Builder {
    pub fn from_entries<R: RngCore + CryptoRng>(prng: &mut R, v: Vec<Entry>) -> Result<Self> {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        for e in &v {
            match e {
                Entry::Issue(e) => {
                    let record = e.to_output_asset_record(prng)?;

                    inputs.push(Input {
                        record: record.clone(),
                        operation: InputOperation::IssueAsset,
                    });

                    outputs.push(Output {
                        record,
                        operation: OutputOperation::IssueAsset,
                        address: Address::from(e.keypair.get_pk()),
                    });
                }
                Entry::Transfer(_t) => {}
            }
        }

        Ok(Builder { inputs, outputs })
    }

    //     pub fn build(&self) -> Result<Transaction> {
    //
    //     }
}
