#![feature(generic_associated_types)]

use abcf::{
    bs3::{
        model::{Map, Value},
        MapStore,
    },
    manager::TContext,
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, RPCResponse, StatefulBatch, StatelessBatch,
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

        let resp = GetAssetOwnerResp { owner };

        RPCResponse::new(resp)
    }
}

/// Module's block logic.
#[abcf::application]
impl Application for CoinbaseModule {
    type Transaction = CoinbaseTransacrion;

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
        println!("{:?}", req.tx);
        for output in &req.tx.outputs {
            log::debug!("Receive coinbase tx: {:?}", &output);
            let owner: XfrPublicKey = output.1.core.public_key;
            let asset_type = match output.1.core.asset_type {
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

            // TODO: this code used to module call, modify in next version of abcf.
            let call_arg = fm_utxo::calls::ArgAddUtxo {
                txid: req.tx.txid.clone(),
                n: output.0,
                output: output.1.clone(),
            };
            context
                .calls
                .push_module_call("utxo", call_arg.to_call_entry());
        }

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl CoinbaseModule {}