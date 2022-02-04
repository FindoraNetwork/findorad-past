use std::collections::BTreeMap;

use abcf::bs3::MapStore;
use libfindora::{asset::Amount, staking::TendermintAddress, Address};
use primitive_types::H160;
use wasmi::{ExternVal, ImportsBuilder, Module, ModuleInstance, ModuleRef, NopExternals};

use crate::{Error, Result};

pub fn version(code: &[u8]) -> Result<H160> {
    let module = Module::from_buffer(code)?;
    let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?.assert_no_start();

    let result = instance
        .invoke_export("_version", &[], &mut NopExternals)?
        .ok_or(Error::VersionNoReturnValue)?;

    let index: u32 = result.try_into().ok_or(Error::ConvertIndexError)?;

    let memory = instance
        .export_by_name("memory")
        .ok_or(Error::NoMemoryExport)?;

    if let ExternVal::Memory(m) = memory {
        let mut res = H160::default();

        m.get_into(index, res.as_bytes_mut())?;

        Ok(res)
    } else {
        Err(Error::NoMemoryExport)
    }
}

pub struct RewardsRuleRuntime {
    instance: ModuleRef,
}

impl RewardsRuleRuntime {
    pub fn new(code: &[u8]) -> Result<Self> {
        let module = Module::from_buffer(code)?;
        let instance = ModuleInstance::new(&module, &ImportsBuilder::default())?.assert_no_start();
        Ok(Self {
            instance,
        })
    }

    pub fn start<D, R>(&mut self, delegator: D, validators: R) -> Result<()>
    where
        D: MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
        R: MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    {
        Ok(())
    }
}
