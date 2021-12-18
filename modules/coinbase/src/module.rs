use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
        MapStore,
    },
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};
use libfindora::{coinbase::CoinbaseTransaction, utxo::OutputId, Address};
use zei::xfr::structs::{AssetType, XfrAssetType};
use fm_utxo::UtxoModule;

#[abcf::module(
    name = "coinbase",
    version = 1,
    impl_version = "0.1.1",
    target_height = 0
)]
#[dependence(utxo = "UtxoModule")]
pub struct CoinbaseModule {
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub asset_owner: Map<AssetType, Address>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub sl_value: Value<u32>,
}

#[abcf::rpcs]
impl CoinbaseModule {}

/// Module's block logic.
#[abcf::application]
impl Application for CoinbaseModule {
    type Transaction = CoinbaseTransaction;

    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        //         for output in &req.tx.outputs {
        //     log::debug!("Receive coinbase tx: {:?}", &output);
        //     let owner = output.1.address.clone();
        //     let asset_type = match output.1.asset {
        //         XfrAssetType::Confidential(_) => {
        //             return Err(abcf::Error::ABCIApplicationError(
        //                 90001,
        //                 String::from("issue asset must be non-confidential"),
        //             ))
        //         }
        //         XfrAssetType::NonConfidential(e) => e,
        //     };
        //
        //     match context.stateful.asset_owner.get(&asset_type)? {
        //         Some(o) => {
        //             if o.as_ref() != &owner {
        //                 return Err(abcf::Error::ABCIApplicationError(
        //                     90002,
        //                     format!(
        //                         "mismatch asset {:?} has owner {:?}, got {:?}",
        //                         asset_type, o, owner
        //                     ),
        //                 ));
        //             }
        //         }
        //         None => {
        //             context.stateful.asset_owner.insert(asset_type, owner)?;
        //         }
        //     }
        //
        //     let utxo = &mut context.deps.utxo;
        //
        //     let output_id = OutputId {
        //         txid: req.tx.txid,
        //         n: output.0,
        //     };
        //
        //     utxo.stateful
        //         .outputs_set
        //         .insert(output_id, output.1.clone())?;
        //     // utxo.stateless.owned_outputs.insert(output.1.address.clone(), output_id)?;
        // }
        //
        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl CoinbaseModule {}
