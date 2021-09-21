use abcf::{
    bs3::{
        model::{Map, Value},
        MapStore,
    },
    manager::TContext,
    module::types::{RequestDeliverTx, ResponseDeliverTx},
    Application, StatefulBatch, StatelessBatch, RPCResponse
};
use libfindora::coinbase::{CoinbaseTransacrion, GetAssetOwnerReq, GetAssetOwnerResp};
use zei::xfr::{
    sig::XfrPublicKey,
    structs::{AssetType, XfrAssetType},
};

#[abcf::module(
    name = "coinbase",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct CoinbaseModule {
    #[stateful]
    pub asset_owner: Map<AssetType, XfrPublicKey>,
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl CoinbaseModule {
    pub async fn get_asset_owner(
        &mut self,
        context: &mut abcf::manager::RContext<'_, abcf::Stateless<Self>, abcf::Stateful<Self>>,
        request: GetAssetOwnerReq,
    ) -> RPCResponse<GetAssetOwnerResp> {
        let asset = request.asset_type;

        let owner = match context.stateful.asset_owner.get(&asset) {
            Err(e) => {
                let error: abcf::Error = e.into();
                return error.into();
            }
            Ok(v) => v.map(|i| i.clone()),
        };

        let resp = GetAssetOwnerResp {
            owner
        };

        RPCResponse::new(resp)
    }
}

/// Module's block logic.
#[abcf::application]
impl Application for CoinbaseModule {
    type Transaction = CoinbaseTransacrion;

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        for output in &req.tx.outputs {
            let owner: XfrPublicKey = output.public_key;
            let asset_type = match output.asset_type {
                XfrAssetType::Confidential(_) => {
                    return Err(abcf::Error::ABCIApplicationError(
                        90001,
                        String::from("issue asset must be non-confidential"),
                    ))
                }
                XfrAssetType::NonConfidential(e) => e,
            };

            match context.stateful.asset_owner.get(&asset_type)? {
                Some(o) => {
                    if o.as_ref() != &owner {
                        return Err(abcf::Error::ABCIApplicationError(
                            90002,
                            format!(
                                "mismatch asset {:?} has owner {:?}, got {:?}",
                                asset_type, o, owner
                            ),
                        ));
                    }
                }
                None => {
                    context.stateful.asset_owner.insert(asset_type, owner)?;
                }
            }

            // insert to utxo.
        }

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl CoinbaseModule {}
