use std::collections::BTreeMap;

use abcf_sdk::providers::Provider;
use libfindora::{
    transaction::{InputOperation, OutputOperation},
    utxo::OutputId,
    Address, Amount,
};
use primitive_types::H512;
use rand_core::{CryptoRng, RngCore};
use ruc::*;
use zei::xfr::structs::{AssetRecord, AssetType};

use crate::Entry;

pub struct Input {
    pub id: OutputId,
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
    pub inputs_amount: BTreeMap<AssetType, Amount>,
    pub outputs_amount: BTreeMap<AssetType, Amount>,
}

impl Builder {
    pub async fn from_entries<R: RngCore + CryptoRng, P: Provider>(
        prng: &mut R,
        provider: P,
        v: Vec<Entry>,
    ) -> Result<Self> {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();

        let mut inputs_amount = BTreeMap::new();
        let mut outputs_amount = BTreeMap::new();

        for e in &v {
            match e {
                Entry::Issue(e) => {
                    let record = e.to_output_asset_record(prng)?;

                    //                     inputs.push(Input {
                    //     id: OutputId {
                    //         txid: H512::zero(),
                    //         n: u32::try_from(inputs.len()).c(d!())?,
                    //     },
                    //     record: record.clone(),
                    //     operation: InputOperation::IssueAsset,
                    // });
                    //
                    // outputs.push(Output {
                    //     record,
                    //     operation: OutputOperation::IssueAsset,
                    //     address: Address::from(e.keypair.get_pk()),
                    // });
                    //
                    // let am = e.to_input_amount()?;
                    //
                    // inputs_amount.insert(am.0, am.1);
                    //
                    // TODO: Add public key for signature.
                }
                Entry::Transfer(t) => {
                    let address = t.to_input_address();
                }
            }
        }

        Ok(Builder {
            inputs,
            outputs,
            inputs_amount,
            outputs_amount,
        })
    }

    //     pub fn build(&self) -> Result<Transaction> {
    //
    //     }
}
