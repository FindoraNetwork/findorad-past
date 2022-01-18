use std::collections::BTreeMap;

use abcf::bs3::MapStore;
use libfindora::{
    asset::{Amount, AssetType, XfrAmount, XfrAssetType},
    utxo::{Output, OutputId},
    Address,
};

use crate::{Error, Result};

/// Read address's balance.(Only Non )
pub fn balance(
    address: Address,
    outputs_sets: &impl MapStore<OutputId, Output>,
    owned_outputs: &impl MapStore<Address, Vec<OutputId>>,
) -> Result<BTreeMap<AssetType, Amount>> {
    let mut amounts: BTreeMap<AssetType, Amount> = BTreeMap::new();

    if let Some(ids) = owned_outputs.get(&address)? {
        for id in ids.as_ref() {
            if let Some(output) = outputs_sets.get(id)? {
                if let (XfrAmount::NonConfidential(amount), XfrAssetType::NonConfidential(asset)) =
                    (&output.amount, &output.asset)
                {
                    if let Some(a) = amounts.get_mut(&asset) {
                        *a = a.checked_add(*amount).ok_or_else(|| Error::AddOverflow)?;
                    } else {
                        amounts.insert(asset.clone(), amount.clone());
                    }
                }
            }
        }
    }

    Ok(amounts)
}
