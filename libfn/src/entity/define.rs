use libfindora::{
    asset::{AssetMeta, AssetType, XfrAmount, XfrAssetType},
    transaction::Output,
    utxo, Address, OutputOperation,
};
use primitive_types::U256;
use serde::{Deserialize, Serialize};
use zei::xfr::sig::XfrKeyPair;

#[derive(Serialize, Deserialize, Debug)]
pub struct Define {
    pub maximum: Option<U256>,
    pub transferable: bool,
    pub keypair: XfrKeyPair,
    pub asset: AssetType,
}

impl Define {
    pub fn to_output(&self) -> Output {
        let address = Address::from(self.keypair.get_pk());

        let core = utxo::Output {
            address,
            asset: XfrAssetType::NonConfidential(self.asset.clone()),
            owner_memo: None,
            amount: XfrAmount::NonConfidential(0),
        };

        let asset = AssetMeta {
            maximum: self.maximum.clone(),
            transferable: self.transferable,
        };

        Output {
            core,
            operation: OutputOperation::DefineAsset(asset),
        }
    }
}
