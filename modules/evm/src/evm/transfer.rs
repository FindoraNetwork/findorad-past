use libfindora::asset::AssetType;
use primitive_types::H160;

pub struct Transfer {
    pub from: H160,
    pub to: H160,
    pub amount: u64,
    pub asset: AssetType,
}
