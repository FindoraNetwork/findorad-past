use std::collections::BTreeSet;

use crate::{entity::Entity, net, utils, Result};
use abcf_sdk::providers::Provider;
use libfindora::{
    transaction::{Input, InputOperation, OutputOperation},
    Address, Transaction,
};
use rand_core::{CryptoRng, RngCore};
use zei::xfr::{
    sig::XfrKeyPair,
    structs::{AssetRecord, BlindAssetRecord},
};

pub struct Output {
    pub record: BlindAssetRecord,
    pub operation: OutputOperation,
    pub address: Address,
}

pub struct Builder {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub zei_inputs: Vec<AssetRecord>,
    pub zei_outputs: Vec<AssetRecord>,
    pub keypairs: Vec<XfrKeyPair>,
}

impl Builder {
    pub async fn from_entries<R: RngCore + CryptoRng, P: Provider>(
        prng: &mut R,
        provider: &mut P,
        v: Vec<Entity>,
    ) -> Result<Self> {
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        // let mut build_outputs = Vec::new();

        let mut zei_inputs = Vec::new();
        let mut zei_outputs = Vec::new();

        let mut from_addresses = BTreeSet::new();

        let mut keypairs = Vec::new();

        for e in &v {
            match e {
                Entity::Issue(e) => {
                    let record = e.to_output_asset_record(prng)?;

                    outputs.push(Output {
                        record: record.open_asset_record.blind_asset_record.clone(),
                        operation: OutputOperation::IssueAsset,
                        address: Address::from(e.keypair.get_pk()),
                    });

                    zei_inputs.push(record);

                    keypairs.push(e.to_keypair());
                }
                Entity::Transfer(t) => {
                    let address = t.to_input_address();
                    let keypair = t.to_keypair();

                    if !from_addresses.contains(&address) {
                        let (ids, outputs) = net::get_owned_outputs(provider, &address)?;
                        from_addresses.insert(address.clone());
                        let mut ars = utils::open_outputs(outputs, &keypair)?;
                        zei_inputs.append(&mut ars);

                        for index in ids {
                            inputs.push(Input {
                                txid: index.txid,
                                n: index.n,
                                operation: InputOperation::TransferAsset,
                            });
                        }
                    }

                    let output = t.to_output_asset_record(prng)?;

                    outputs.push(Output {
                        record: output.open_asset_record.blind_asset_record.clone(),
                        operation: OutputOperation::TransferAsset,
                        address,
                    });

                    zei_outputs.push(output);
                }
            }
        }

        // Generate fee.
        let fee_ar = utils::build_fee(prng)?;
        outputs.push(Output {
            address: Address::BlockHole,
            record: fee_ar.open_asset_record.blind_asset_record.clone(),
            operation: OutputOperation::Fee,
        });
        zei_outputs.push(fee_ar);

        Ok(Builder {
            inputs,
            outputs,
            zei_inputs,
            zei_outputs,
            keypairs,
        })
    }

    pub fn build(&self) -> Result<Transaction> {
        // 1. check 
        // 2. change
        // 3. build xfr body.
        // 4. build transaction.
        // 5. signature.
        Ok(Default::default())
    }
}
