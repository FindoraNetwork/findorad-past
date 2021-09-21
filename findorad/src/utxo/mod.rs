use std::convert::TryInto;

use abcf::{
    bs3::{
        model::{Map, Value},
        MapStore,
    },
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, Event, StatefulBatch, StatelessBatch,
};
use libfindora::{utxo::{OutputId, UtxoTransacrion, ValidateTransaction}};
use rand_chacha::ChaChaRng;
use serde::{Deserialize, Serialize};
use zei::{setup::PublicParams, xfr::{structs::BlindAssetRecord}};

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UtxoModule {
    params: PublicParams,
    prng: ChaChaRng,
    #[stateful]
    pub output_set: Map<OutputId, BlindAssetRecord>,
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl UtxoModule {}

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
                return Err(abcf::Error::ABCIApplicationError(90001, String::from("Output doesn't exists.")));
            }
        }

        let result = validate_tx.verify(&mut self.prng, &mut self.params);

        match result {
            Ok(_) => {
                // TODO: May panic when access store.
                for input in &tx.inputs {
                    context.stateful.output_set.remove(input)?;
                }
                for i in 0 .. tx.outputs.len() {
                    let output = &tx.outputs[i];

                    let output_id = OutputId {
                        txid: tx.txid.clone(),
                        n: i.try_into().map_err(|e| {
                            abcf::Error::ABCIApplicationError(90003, format!("{}", e))
                        })?,
                    };

                    context.stateful.output_set.insert(output_id, output.clone())?;
                }
            }
            Err(e) => {
                abcf::Error::ABCIApplicationError(90002, format!("{}", e));
            }
        }
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl UtxoModule {}
