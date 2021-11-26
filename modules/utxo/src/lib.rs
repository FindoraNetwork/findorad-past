#![feature(generic_associated_types)]

use std::convert::TryInto;

use abcf::{
    bs3::{model::Map, MapStore},
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCResponse, StatefulBatch, StatelessBatch,
};
use libfindora::utxo::{
    Address, GetOwnedUtxoReq, GetOwnedUtxoResp, Output, OutputId, OwnedOutput, UtxoTransacrion,
    ValidateTransaction,
};
use rand_chacha::ChaChaRng;
use zei::setup::PublicParams;

pub mod calls;
// mod event;

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UtxoModule {
    params: PublicParams,
    prng: ChaChaRng,
    #[stateful]
    pub output_set: Map<OutputId, Output>,
    #[stateless]
    pub owned_outputs: Map<Address, Vec<OutputId>>,
}

#[abcf::rpcs]
impl UtxoModule {
    pub async fn get_owned_outputs(
        &mut self,
        context: &mut abcf::manager::RContext<'_, abcf::Stateless<Self>, abcf::Stateful<Self>>,
        request: GetOwnedUtxoReq,
    ) -> RPCResponse<GetOwnedUtxoResp> {
        let mut outputs = Vec::new();

        for owner_id in 0..request.owners.len() {
            let owner = &request.owners[owner_id];
            match context.stateless.owned_outputs.get(owner) {
                Err(e) => {
                    let error: abcf::Error = e.into();
                    return error.into();
                }
                Ok(v) => match v {
                    Some(s) => {
                        let output_ids = s.as_ref();
                        for output_id in output_ids {
                            if let Ok(Some(output)) = context.stateful.output_set.get(output_id) {
                                outputs.push((
                                    owner_id,
                                    OwnedOutput {
                                        output_id: output_id.clone(),
                                        output: output.clone(),
                                    },
                                ))
                            }
                        }
                    }
                    None => {}
                },
            };
        }
        let resp = GetOwnedUtxoResp { outputs };
        RPCResponse::new(resp)
    }
}

/// Module's block logic.
#[abcf::application]
impl Application for UtxoModule {
    type Transaction = UtxoTransacrion;

    async fn check_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        // TODO: this code used to module call, modify in next version of abcf.
        if let Some(calls) = context.calls.pop_module_calls("utxo") {
            for entry in calls {
                match entry.method.as_str() {
                    "add_utxo" => {
                        let args = entry.args.downcast::<calls::ArgAddUtxo>();

                        if args.is_ok() {
                            let args = *args.unwrap();
                            let output_id = OutputId {
                                txid: args.txid,
                                n: args.n,
                            };
                            let output = args.output;

                            let owner = output.address.clone();
                            match context.stateless.owned_outputs.get_mut(&owner)? {
                                Some(v) => {
                                    v.push(output_id.clone());
                                }
                                None => {
                                    let mut v = Vec::new();
                                    v.push(output_id.clone());
                                    context.stateless.owned_outputs.insert(owner, v)?;
                                }
                            }
                            context.stateful.output_set.insert(output_id, output)?;
                        }
                    }
                    _ => {}
                }
            }
        }

        let tx: &UtxoTransacrion = &req.tx;

        let mut validate_tx = ValidateTransaction {
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: tx.proof.clone(),
        };

        for input in &tx.inputs {
            let record = context.stateful.output_set.remove(input)?;
            if let Some(r) = record {
                validate_tx.inputs.push(r.clone().to_blind_asset_record());
                if let Some(owned_outputs) = context.stateless.owned_outputs.get_mut(&r.address)? {
                    if let Some(index) = owned_outputs
                        .iter()
                        .position(|x| x.txid == input.txid && x.n == input.n)
                    {
                        owned_outputs.remove(index);
                    }
                }
            } else {
                return Err(abcf::Error::ABCIApplicationError(
                    90001,
                    String::from("Output doesn't exists."),
                ));
            }
        }

        for output in &tx.outputs {
            validate_tx
                .outputs
                .push(output.clone().to_blind_asset_record());
        }

        let result = validate_tx.verify(&mut self.prng, &mut self.params);

        match result {
            Ok(_) => {
                for i in 0..tx.outputs.len() {
                    let output = &tx.outputs[i];
                    let txid = &tx.txid;
                    let n = i
                        .try_into()
                        .map_err(|e| abcf::Error::ABCIApplicationError(90003, format!("{}", e)))?;

                    let output_id = OutputId {
                        txid: *txid,
                        n,
                    };

                    context
                        .stateful
                        .output_set
                        .insert(output_id.clone(), output.clone())?;

                    let owner = output.address.clone();
                    match context.stateless.owned_outputs.get_mut(&owner)? {
                        Some(v) => {
                            v.push(output_id);
                            log::debug!("Current list is: {:?}", v);
                        }
                        None => {
                            let mut v = Vec::new();
                            v.push(output_id);
                            context.stateless.owned_outputs.insert(owner, v)?;
                        }
                    }
                }
            }
            Err(e) => {
                return Err(abcf::Error::ABCIApplicationError(90002, format!("{}", e)));
            }
        }

        //         // 1. recv events
        // for input in validate_tx.inputs {
        //     let e = event::SendEvent::new_from_record(&input);
        //     context.events.emmit(e)?;
        // }
        // // 2. send events
        // for output in validate_tx.outputs {
        //     let e = event::RecvEvent::new_from_record(&output);
        //     context.events.emmit(e)?;
        //         }

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl UtxoModule {}

pub mod utxo_module_rpc {
    include!(concat!(env!("OUT_DIR"), "/utxomodule.rs"));
}
