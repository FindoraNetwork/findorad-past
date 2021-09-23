use std::convert::TryInto;

use abcf::{
    bs3::{model::Map, MapStore},
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCResponse, StatefulBatch, StatelessBatch,
};
use libfindora::utxo::{GetOwnedUtxoReq, GetOwnedUtxoResp, OutputId, OwnedOutput, UtxoTransacrion, ValidateTransaction};
use rand_chacha::ChaChaRng;
use zei::{
    setup::PublicParams,
    xfr::{sig::XfrPublicKey, structs::BlindAssetRecord},
    serialization::ZeiFromToBytes,
};

pub mod calls;

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UtxoModule {
    params: PublicParams,
    prng: ChaChaRng,
    #[stateful]
    pub output_set: Map<OutputId, BlindAssetRecord>,
    #[stateless]
    pub owned_outputs: Map<XfrPublicKey, Vec<OwnedOutput>>,
}

#[abcf::rpcs]
impl UtxoModule {
    pub async fn get_owned_outputs(
        &mut self,
        context: &mut abcf::manager::RContext<'_, abcf::Stateless<Self>, abcf::Stateful<Self>>,
        request: GetOwnedUtxoReq,
    ) -> RPCResponse<GetOwnedUtxoResp> {
        match XfrPublicKey::zei_from_bytes(request.owner.as_ref()) {

            Err(e) => {
                let error = abcf::Error::RPCApplicationError(90001, format!("{:?}", e));
                error.into()
            }
            Ok(owner) => {
                let outputs = match context.stateless.owned_outputs.get(&owner) {
                    Err(e) => {
                        let error: abcf::Error = e.into();
                        return error.into();
                    }
                    Ok(v) => match v {
                        Some(s) => s.clone(),
                        None => Vec::new(),
                    },
                };

                let resp = GetOwnedUtxoResp { outputs };

                RPCResponse::new(resp)
            }
        }
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

//                             context.stateless.owned_outputs.insert(output.public_key.clone(), OwnedOutput {
                                // txid: output_id.txid.clone(),
                                // n: output_id.n,
                                // core: output.clone(),
//                             });
                            context.stateful.output_set.insert(output_id, output)?;
                        }
                    }
                    _ => {}
                }
            }
        }

        let tx = &req.tx;

        let mut validate_tx = ValidateTransaction {
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: tx.proof.clone(),
        };

        for input in &tx.inputs {
            let record = context.stateful.output_set.get(input)?;
            if let Some(r) = record {
                validate_tx.inputs.push(r.clone());
            } else {
                return Err(abcf::Error::ABCIApplicationError(
                    90001,
                    String::from("Output doesn't exists."),
                ));
            }
        }

        let result = validate_tx.verify(&mut self.prng, &mut self.params);

        match result {
            Ok(_) => {
                for input in &tx.inputs {
                    context.stateful.output_set.remove(input)?;
                }
                for i in 0..tx.outputs.len() {
                    let output: &BlindAssetRecord = &tx.outputs[i];
                    let txid = &tx.txid;
                    let n = i.try_into().map_err(|e| {
                            abcf::Error::ABCIApplicationError(90003, format!("{}", e))
                        })?;

                    let output_id = OutputId {
                        txid: txid.clone(),
                        n,
                    };

                    context
                        .stateful
                        .output_set
                        .insert(output_id, output.clone())?;

                    let owner = output.public_key;
                    match context.stateless.owned_outputs.get_mut(&owner)? {
                        Some(v) => {
                            v.push(OwnedOutput {
                                core: output.clone(),
                                txid: txid.clone(),
                                n
                            });
                        }
                        None => {
                            let mut v = Vec::new();
                            v.push(OwnedOutput {
                                core: output.clone(),
                                txid: txid.clone(),
                                n
                            });
                            context.stateless.owned_outputs.insert(owner, v)?;
                        }
                    }
                }
            }
            Err(e) => {
                return Err(abcf::Error::ABCIApplicationError(90002, format!("{}", e)));
            }
        }
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl UtxoModule {}
