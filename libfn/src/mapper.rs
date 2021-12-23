use std::collections::BTreeMap;

use libfindora::{
    asset::{Amount, AssetType},
    Address,
};

use crate::{Error, Result};

#[derive(Debug, Default)]
pub struct Mapper {
    pub inner: BTreeMap<Address, BTreeMap<AssetType, (Amount, bool, bool)>>,
}

impl Mapper {
    pub fn add(
        &mut self,
        address: &Address,
        asset: &AssetType,
        amount: Amount,
        confidential_amount: bool,
        confidential_asset: bool,
    ) -> Result<()> {
        if let Some(v) = self.inner.get_mut(address) {
            if let Some((a, ca, ct)) = v.get_mut(asset) {
                *a = a.checked_add(amount).ok_or(Error::OverflowAdd)?;
                *ca = *ca || confidential_amount;
                *ct = *ct || confidential_asset;
            } else {
                v.insert(*asset, (amount, false, false));
            }
        } else {
            let mut v = BTreeMap::new();

            v.insert(*asset, (amount, false, false));

            self.inner.insert(address.clone(), v);
        }
        Ok(())
    }

    pub fn sub(
        &mut self,
        address: &Address,
        asset: &AssetType,
        amount: Amount,
        confidential_amount: bool,
        confidential_asset: bool,
    ) -> Result<()> {
        if let Some(v) = self.inner.get_mut(address) {
            if let Some((a, ca, ct)) = v.get_mut(asset) {
                *a = a.checked_sub(amount).ok_or(Error::BalanceNotEnough)?;
                *ca = *ca || confidential_amount;
                *ct = *ct || confidential_asset;
            } else {
                return Err(Error::BalanceNotEnough);
            }
        } else {
            return Err(Error::BalanceNotEnough);
        }
        Ok(())
    }

    pub fn to_vec(self) -> Vec<(Address, AssetType, Amount, bool, bool)> {
        let mut v = Vec::new();

        for (address, aa_mapper) in self.inner {
            for (asset, amount) in aa_mapper {
                v.push((address.clone(), asset, amount.0, amount.1, amount.2));
            }
        }

        v
    }
}
