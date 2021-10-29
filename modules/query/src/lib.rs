#![feature(generic_associated_types)]

mod error;

use abcf::{
    bs3::{model::Map, MapStore, model::Vec as bs3_vec},
    manager::TContext,
    module::types::{RequestDeliverTx, ResponseDeliverTx},
    Application, RPCResponse, StatefulBatch, StatelessBatch,
};
use abcf::bs3::model::Value;
use libfindora::utxo::{
    GetOwnedUtxoReq, GetOwnedUtxoResp, OutputId,
};
use zei::xfr::sig::XfrPublicKey;
use zei::serialization::ZeiFromToBytes;
use libfindora::common::{AssetType, AssetTypeCode, DefineAsset, Issuances, QueryTxOutPut, StateCommitmentData};
use libfindora::query::QueryTransaction;
use libfindora::query::rpc::{GetAssetTypeReq, GetAssetTypeResp, GetOwnedAssetReq, GetOwnedAssetResp, GetTokenCodeReq, GetTokenCodeResp};
use libfindora::transaction::{InputOperation, OutputOperation};
use hex_literal::hex;
use sha3::{Digest, Sha3_256};

#[abcf::module(name = "query", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct QueryModule {
    #[stateful]
    pub none: Value<u8>,
    #[stateless]
    pub owned_asset: Map<XfrPublicKey, Vec<DefineAsset>>,
    #[stateless]
    pub asset_types: Map<AssetTypeCode, AssetType>,
    #[stateless]
    pub token_code: Map<AssetTypeCode, Issuances>,
    #[stateless]
    pub state_commitment_versions: bs3_vec<StateCommitmentData>,
}

#[abcf::rpcs]
impl QueryModule {

    pub async fn get_owned_asset(
        &mut self,
        context: &mut abcf::manager::RContext<'_, abcf::Stateless<Self>, abcf::Stateful<Self>>,
        request: GetOwnedAssetReq,
    ) -> RPCResponse<GetOwnedAssetResp> {
        let mut resp = Vec::new();

        for i in 0..request.owner.len() {
            let owner = &request.owner[i];
            match context.stateless.owned_asset.get(owner) {
                Err(e) => {
                    let error: abcf::Error = e.into();
                    return error.into();
                }
                Ok(v) => match v {
                    None => {}
                    Some(s) => {
                        let base64_pub_key = base64::encode(&owner.zei_to_bytes());
                        resp.push((base64_pub_key,s.clone()));
                    }
                }
            };
        }

        RPCResponse::new(GetOwnedAssetResp{ resp })
    }

    pub async fn get_asset_types(
        &mut self,
        context: &mut abcf::manager::RContext<'_, abcf::Stateless<Self>, abcf::Stateful<Self>>,
        request: GetAssetTypeReq,
    ) -> RPCResponse<GetAssetTypeResp> {
        let mut resp = Vec::new();

        for i in 0..request.asset_type_code.len() {
            let code = &request.asset_type_code[i];
            match context.stateless.asset_types.get(code) {
                Err(e) => {
                    let error: abcf::Error = e.into();
                    return error.into();
                }
                Ok(v) => match v {
                    None => {resp.push(None)}
                    Some(s) => {
                        resp.push(Some(s.clone()));
                    }
                }
            }
        }

        RPCResponse::new(GetAssetTypeResp{ resp })
    }

    pub async fn get_token_code(
        &mut self,
        context: &mut abcf::manager::RContext<'_, abcf::Stateless<Self>, abcf::Stateful<Self>>,
        request: GetTokenCodeReq,
    ) -> RPCResponse<GetTokenCodeResp> {
        let resp;

        match context.stateless.token_code.get(&request.asset_type_code) {
            Err(e) => {
                let error: abcf::Error = e.into();
                return error.into();
            }
            Ok(v) => match v {
                None => {resp = None}
                Some(s) => {
                    resp = Some(s.clone())
                }
            }
        }

        RPCResponse::new(GetTokenCodeResp{ resp })
    }
}

#[abcf::application]
impl Application for QueryModule {
    type Transaction = QueryTransaction;

    async fn deliver_tx(
        &mut self,
        context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {

        let qt: &QueryTransaction = &req.tx;


        for (index,output) in qt.tx.outputs.iter().enumerate() {
            match output.operation {
                OutputOperation::TransferAsset => {}
                OutputOperation::IssueAsset => {
                    let pubkey = output.core.public_key;
                    let define_asset = DefineAsset::new_from_output(output)
                        .map_err(|e|abcf::Error::ABCIApplicationError(60709,e.to_string()))?;
                    match context.stateless.owned_asset.get_mut(&pubkey)? {
                        Some(v) => {
                            v.push(define_asset.clone());
                        }
                        None => {
                            let mut v = Vec::new();
                            v.push(define_asset.clone());
                            context.stateless.owned_asset.insert(pubkey, v)?;
                        }
                    }

                    let asset_type_code = define_asset.body.asset_type_code.clone();
                    match context.stateless.asset_types.get(&asset_type_code)? {
                        Some(_) => {
                            let type_code = serde_json::to_string(&asset_type_code)?;
                            return Err(abcf::Error::ABCIApplicationError(60710,format!("AssetTypeCode:{} already exists",type_code)));
                        }
                        None => {
                            let asset_type = AssetType::new_from_define_asset(&define_asset)
                                .map_err(|e|abcf::Error::ABCIApplicationError(60711,e.to_string()))?;
                            context.stateless.asset_types.insert(asset_type_code.clone(), asset_type)?;
                        }
                    }

                    let input = &qt.tx.inputs[index];
                    if input.operation != InputOperation::IssueAsset {
                        return Err(abcf::Error::ABCIApplicationError(60712,format!("the index input mismatch:{:?} ",input)));
                    }

                    if input.n as usize != index {
                        return Err(abcf::Error::ABCIApplicationError(60713,format!("the index mismatch output index")));
                    }

                    let query_tx_output = QueryTxOutPut::new_from_input_and_output(output, input)
                        .map_err(|e|abcf::Error::ABCIApplicationError(60714,e.to_string()))?;

                    match context.stateless.token_code.get_mut(&asset_type_code)? {
                        Some(v) => {
                            v.push((query_tx_output, None));
                        }
                        None => {
                            let mut v = Vec::new();
                            v.push((query_tx_output, None));
                            context.stateless.token_code.insert(asset_type_code.clone(), v)?;
                        }
                    }

                }
                OutputOperation::Fee => {}
                OutputOperation::Delegate(_) => {}
                OutputOperation::ClaimReward(_) => {}
                OutputOperation::Undelegate(_) => {}
            }
        }

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl QueryModule {}

pub mod query_module_rpc {
    include!(concat!(env!("OUT_DIR"), "/querymodule.rs"));
}