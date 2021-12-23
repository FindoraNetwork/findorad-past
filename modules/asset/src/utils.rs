use abcf::bs3::MapStore;
use libfindora::asset::AssetType;

use crate::{AssetInfo, AssetIssue, Error, Result};

pub fn check_define(
    asset_infos: &mut impl MapStore<AssetType, AssetInfo>,
    tx: &[AssetInfo],
) -> Result<()> {
    for define in tx {
        if asset_infos.get(&define.asset)?.is_some() {
            return Err(Error::AssetTypeAlreadyExists(define.asset));
        }
        asset_infos.insert(define.asset, define.clone())?;
    }

    Ok(())
}

pub fn check_issue(
    asset_infos: &impl MapStore<AssetType, AssetInfo>,
    tx: &[AssetIssue],
) -> Result<()> {
    for issue in tx {
        if let Some(info) = asset_infos.get(&issue.asset)? {
            if info.owner != issue.address {
                return Err(Error::IssueMustBeOwner(
                    issue.address.clone(),
                    info.owner.clone(),
                ));
            }

            // If confidnetal amount and have maximum, failed.
            if info.maximum.is_some() && issue.amount.is_confidential() {
                return Err(Error::MustBeNonConfidentialAmount);
            }

            // TODO: check maximum.
        } else {
            return Err(Error::AssetTypeNotExists(issue.asset));
        }
    }

    Ok(())
}

pub fn check_transfer(
    asset_infos: &impl MapStore<AssetType, AssetInfo>,
    tx: &[AssetType],
) -> Result<()> {
    for asset in tx {
        if let Some(a) = asset_infos.get(asset)? {
            if !a.transferable {
                return Err(Error::AssetCantTransfer(*asset));
            }
        } else {
            return Err(Error::AssetTypeNotExists(*asset));
        }
    }

    Ok(())
}
