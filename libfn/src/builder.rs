use std::collections::BTreeMap;

use crate::{entity::Entity, mapper::Mapper, net, utils, Error, Result};
use abcf_sdk::providers::Provider;
use libfindora::{
    asset::FRA,
    transaction::{Input, InputOperation, Output, OutputOperation},
    utxo, Address, Transaction,
};
use primitive_types::H512;
use rand_core::{CryptoRng, RngCore};
use zei::xfr::{lib::gen_xfr_body, sig::XfrKeyPair, structs::AssetRecord};
use zei::xfr::structs::{AssetTypeAndAmountProof, XfrBody, XfrProofs};

#[derive(Debug, Default)]
pub struct Builder {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub zei_inputs: Vec<AssetRecord>,
    pub zei_outputs: Vec<AssetRecord>,
    pub keypairs: BTreeMap<Address, XfrKeyPair>,
    pub mapper: Mapper,
}

impl Builder {
    pub async fn fetch_owned_utxo<P: Provider>(
        &mut self,
        provider: &mut P,
        address: &Address,
        keypair: &XfrKeyPair,
    ) -> Result<()> {
        if !self.keypairs.contains_key(address) {
            let (ids, outputs) = net::get_owned_outputs(provider, address).await?;

            let mut ars = utils::open_outputs(outputs, keypair)?;

            for ar in &ars {
                self.mapper.add(
                    address,
                    &ar.open_asset_record.asset_type,
                    ar.open_asset_record.amount,
                    false,
                    false,
                )?;
            }

            self.zei_inputs.append(&mut ars);

            for index in ids {
                self.inputs.push(Input {
                    txid: index.txid,
                    n: index.n,
                    operation: InputOperation::TransferAsset,
                });
            }

            self.keypairs.insert(address.clone(), keypair.clone());
        }
        Ok(())
    }

    pub async fn from_entities<R: RngCore + CryptoRng, P: Provider>(
        &mut self,
        prng: &mut R,
        provider: &mut P,
        v: Vec<Entity>,
    ) -> Result<()> {
        for e in &v {
            match e {
                Entity::Define(e) => {
                    let output = e.to_output();
                    self.outputs.push(output);
                }
                Entity::Issue(e) => {
                    let record = e.to_output_asset_record(prng)?;

                    let address = Address::from(e.keypair.get_pk());
                    let keypair = e.to_keypair();

                    self.fetch_owned_utxo(provider, &address, &keypair).await?;

                    self.mapper.add(
                        &address,
                        &e.asset_type,
                        e.amount,
                        e.is_confidential(),
                        false,
                    )?;

                    let core = utxo::Output {
                        amount: record.open_asset_record.blind_asset_record.amount.clone(),
                        asset: record
                            .open_asset_record
                            .blind_asset_record
                            .asset_type
                            .clone(),
                        address: address.clone(),
                        owner_memo: record.owner_memo.clone(),
                    };

                    self.outputs.push(Output {
                        operation: OutputOperation::IssueAsset,
                        core,
                    });

                    self.zei_inputs.push(record);

                    let mut index:u32 = self.outputs.len().try_into()?;
                    index = index -1;

                    self.inputs.push(Input {
                        txid: primitive_types::H512::zero(),
                        n: index,
                        operation: InputOperation::TransferAsset,
                    });

                    self.keypairs.insert(address, keypair);
                }
                Entity::Transfer(t) => {
                    let address = t.to_input_address();
                    let keypair = t.to_keypair();

                    let record = t.to_output_asset_record(prng)?;

                    self.fetch_owned_utxo(provider, &address, &keypair).await?;

                    self.mapper.sub(
                        &address,
                        &record.open_asset_record.asset_type,
                        record.open_asset_record.amount,
                        t.is_confidential_amount(),
                        t.is_confidential_asset(),
                    )?;

                    let core = utxo::Output {
                        amount: record.open_asset_record.blind_asset_record.amount.clone(),
                        asset: record
                            .open_asset_record
                            .blind_asset_record
                            .asset_type
                            .clone(),
                        address:t.address.clone(),
                        owner_memo: record.owner_memo.clone(),
                    };

                    self.outputs.push(Output {
                        operation: OutputOperation::TransferAsset,
                        core,
                    });

                    self.zei_outputs.push(record);
                }
                Entity::Delegate(d) => {
                    let output = d.to_output(prng)?;

                    let address = Address::from(d.keypair.get_pk());

                    let bar = &output.open_asset_record.blind_asset_record;

                    let keypair = d.to_keypair();

                    self.fetch_owned_utxo(provider, &address, &keypair).await?;

                    self.mapper.sub(
                        &address,
                        &output.open_asset_record.asset_type,
                        output.open_asset_record.amount,
                        false,
                        false,
                    )?;

                    let core = utxo::Output {
                        amount: bar.amount.clone(),
                        asset: bar.asset_type.clone(),
                        owner_memo: None,
                        address: address.clone(),
                    };

                    self.zei_outputs.push(output);

                    let output = Output {
                        core,
                        operation: OutputOperation::Delegate(d.to_operation()?),
                    };

                    self.outputs.push(output);

                    self.keypairs.insert(address, keypair);
                }
                Entity::Stake(d) => {
                    let output = d.to_output(prng)?;

                    let address = Address::from(d.keypair.get_pk());

                    let bar = &output.open_asset_record.blind_asset_record;

                    let keypair = d.to_keypair();

                    self.fetch_owned_utxo(provider, &address, &keypair).await?;

                    self.mapper.sub(
                        &address,
                        &output.open_asset_record.asset_type,
                        output.open_asset_record.amount,
                        false,
                        false,
                    )?;

                    let core = utxo::Output {
                        amount: bar.amount.clone(),
                        asset: bar.asset_type.clone(),
                        owner_memo: None,
                        address: address.clone(),
                    };

                    self.zei_outputs.push(output);

                    let output = Output {
                        core,
                        operation: OutputOperation::Delegate(d.to_operation()?),
                    };

                    self.outputs.push(output);

                    self.keypairs.insert(address, keypair);
                }

                Entity::Undelegate(e) => {
                    let record = e.to_output(prng)?;

                    let address = Address::from(e.keypair.get_pk());
                    let keypair = e.to_keypair();

                    self.fetch_owned_utxo(provider, &address, &keypair).await?;

                    self.mapper
                        .add(&address, &FRA.bare_asset_type, e.amount, false, false)?;

                    let core = utxo::Output {
                        amount: record.open_asset_record.blind_asset_record.amount.clone(),
                        asset: record
                            .open_asset_record
                            .blind_asset_record
                            .asset_type
                            .clone(),
                        address: address.clone(),
                        owner_memo: record.owner_memo.clone(),
                    };

                    self.outputs.push(Output {
                        operation: OutputOperation::IssueAsset,
                        core,
                    });

                    self.zei_inputs.push(record);

                    self.inputs.push(Input {
                        txid: primitive_types::H512::zero(),
                        n: self.outputs.len().try_into()?,
                        operation: InputOperation::TransferAsset,
                    });

                    self.keypairs.insert(address, keypair);
                }
            }
        }

        Ok(())
    }

    pub fn build<R: RngCore + CryptoRng>(mut self, prng: &mut R) -> Result<Transaction> {
        // Generate fee.
        let record = utils::build_fee(prng)?;

        let core = utxo::Output {
            amount: record.open_asset_record.blind_asset_record.amount.clone(),
            asset: record
                .open_asset_record
                .blind_asset_record
                .asset_type
                .clone(),
            address: Address::blockhole(),
            owner_memo: record.owner_memo.clone(),
        };

        let output = Output {
            core,
            operation: OutputOperation::Fee,
        };

        /// add a handling fee for each operation initiator
        let addr_vec = self.mapper.inner.iter().map(|(addr,_)|{
            addr.clone()
        }).collect::<Vec<Address>>();
        for addr in addr_vec.iter() {
            self.mapper.sub(
                addr,
                &record.open_asset_record.asset_type,
                record.open_asset_record.amount,
                false,
                false,
            )?;
        }

        self.outputs.push(output);
        self.zei_outputs.push(record);

        // change

        let mapper_vec = self.mapper.to_vec();

        log::debug!("Charge is {:?}", mapper_vec);

        for (address, asset, amount, confidential_amount, confidential_asset) in mapper_vec {
            let public_key = self
                .keypairs
                .get(&address)
                .ok_or(Error::BalanceNotEnough)?
                .get_pk();

            let record = utils::build_output(
                prng,
                asset,
                amount,
                confidential_asset,
                confidential_amount,
                public_key,
            )?;

            /// get change for your own utxo
            let core = utxo::Output {
                amount: record.open_asset_record.blind_asset_record.amount.clone(),
                asset: record
                    .open_asset_record
                    .blind_asset_record
                    .asset_type
                    .clone(),
                address,
                owner_memo: record.owner_memo.clone(),
            };
            self.outputs.push(Output {
                core,
                operation: OutputOperation::TransferAsset,
            });

            self.zei_outputs.push(record);
        }

        // build xfr body.

        let body = if self.zei_inputs.len() != 0 {
            gen_xfr_body(prng, &self.zei_inputs, &self.zei_outputs)?
        } else {
            XfrBody{
                inputs: vec![],
                outputs: vec![],
                proofs: XfrProofs { asset_type_and_amount_proof: AssetTypeAndAmountProof::NoProof, asset_tracing_proof: Default::default() },
                asset_tracing_memos: vec![],
                owners_memos: vec![],
                input_public_keys: vec![]
            }
        };

        // build transaction.

        let mut tx = Transaction {
            txid: H512::default(),
            inputs: self.inputs,
            outputs: self.outputs,
            proof: body.proofs.asset_type_and_amount_proof,
            signatures: Vec::new(),
        };

        // signature.
        let keypairs = self.keypairs.into_values().collect::<Vec<XfrKeyPair>>();
        tx.signature(&keypairs)?;

        Ok(tx)
    }
}
