use abcf::bs3::MapStore;
use libfindora::{
    asset::{Amount, AssetType, XfrAmount, XfrAssetType},
    utxo::{Output, OutputId},
    Address,
};

use crate::{Error, Result};

pub fn mint(
    to: Address,
    amount: Amount,
    asset: AssetType,
    oid: OutputId,
    outputs_sets: &mut impl MapStore<OutputId, Output>,
    owned_outputs: &mut impl MapStore<Address, Vec<OutputId>>,
) -> Result<()> {
    let output = Output {
        address: to.clone(),
        amount: XfrAmount::NonConfidential(amount),
        asset: XfrAssetType::NonConfidential(asset),
        owner_memo: None,
    };

    outputs_sets.insert(oid.clone(), output)?;

    if let Some(v) = owned_outputs.get_mut(&to)? {
        v.push(oid);
    } else {
        let v = vec![oid];
        owned_outputs.insert(to.clone(), v)?;
    }

    Ok(())
}

pub fn burn(
    from: Address,
    amount: Amount,
    asset: AssetType,
    outputs_sets: &mut impl MapStore<OutputId, Output>,
    owned_outputs: &mut impl MapStore<Address, Vec<OutputId>>,
) -> Result<()> {
    let mut target_amount = amount;

    if let Some(ids) = owned_outputs.get_mut(&from)? {
        while let Some(id) = ids.pop() {
            if let Some(output) = outputs_sets.get_mut(&id)? {
                if let (XfrAmount::NonConfidential(am), XfrAssetType::NonConfidential(at)) =
                    (&mut output.amount, &output.asset)
                {
                    if at == &asset {
                        if *am < target_amount {
                            target_amount =
                                target_amount.checked_sub(*am).ok_or(Error::SubOverflow)?;
                            // remove output.
                            outputs_sets.remove(&id)?;
                        } else {
                            *am = am.checked_sub(target_amount).ok_or(Error::SubOverflow)?;
                        }
                    }
                }
            }
        }
    }

    if target_amount != 0 {
        return Err(Error::InsufficientBalance);
    }

    Ok(())
}

pub fn transfer(
    from: Address,
    to: Address,
    amount: Amount,
    asset: AssetType,
    oid: OutputId,
    outputs_sets: &mut impl MapStore<OutputId, Output>,
    owned_outputs: &mut impl MapStore<Address, Vec<OutputId>>,
) -> Result<()> {
    burn(from, amount, asset, outputs_sets, owned_outputs)?;

    mint(to, amount, asset, oid, outputs_sets, owned_outputs)?;

    Ok(())
}
