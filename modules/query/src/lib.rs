#![feature(generic_associated_types)]

use std::convert::TryInto;

use abcf::{
    bs3::{model::Map, MapStore, model::Vec},
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCResponse, StatefulBatch, StatelessBatch,
};
use libfindora::utxo::{
    GetOwnedUtxoReq, GetOwnedUtxoResp, Output, OutputId, OwnedOutput, UtxoTransacrion,
    ValidateTransaction,
};
use libfindora::event;
use rand_chacha::ChaChaRng;
use zei::{setup::PublicParams, xfr::sig::XfrPublicKey};
use zei::xfr::structs::AssetRecord;
use libfindora::common::{AssetType, AssetTypeCode, DefineAsset, Issuances, StateCommitmentData};

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct QueryModule {
    params: PublicParams,
    prng: ChaChaRng,
    #[stateful]
    pub output_set: Map<OutputId, Output>,
    #[stateless]
    pub owned_outputs: Map<XfrPublicKey, Vec<OutputId>>,
    #[stateless]
    pub owned_asset: Map<XfrPublicKey, Vec<DefineAsset>>,
    #[stateless]
    pub asset_types: Map<XfrPublicKey, AssetType>,
    #[stateless]
    pub token_code: Map<AssetTypeCode, Issuances>,
    #[stateless]
    pub state_commitment_versions: Vec<StateCommitmentData>,
}

#[abcf::rpcs]
impl QueryModule {
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
                            if let Ok(Some(output)) = context.stateful.output_set.get(&output_id) {
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

#[abcf::application]
impl Application for QueryModule {
    type Transaction = UtxoTransacrion;

    async fn deliver_tx(
        &mut self,
        context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        Ok(Default::default())
    }
}