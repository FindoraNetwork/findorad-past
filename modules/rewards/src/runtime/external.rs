use std::collections::BTreeMap;

use abcf::bs3::MapStore;
use libfindora::{asset::Amount, staking::TendermintAddress, Address};
use wasmi::{MemoryRef, RuntimeArgs, RuntimeValue, Trap};

pub struct External<'a, D, R> {
    pub delegator: &'a mut D,
    pub rewards: &'a mut R,
    pub memory: MemoryRef,
}

impl<'a, D, R> wasmi::Externals for External<'a, D, R>
where
    D: MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
    R: MapStore<TendermintAddress, BTreeMap<Address, Amount>>,
{
    fn invoke_index(
        &mut self,
        _index: usize,
        _args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        Ok(None)
    }
}
