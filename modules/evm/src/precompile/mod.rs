use std::collections::BTreeMap;

use evm::executor::stack::{PrecompileFn, PrecompileSet, PrecompileOutput, PrecompileFailure, };
use precompile_core::Precompile;
use precompile_simple::{Sha256, ECRecover, Ripemd160, Identity};
use primitive_types::H160;

pub struct Precompiles {
    pub set: BTreeMap<H160, PrecompileFn>,
}

impl Precompiles {
    pub fn new() -> Self {
        let mut set = BTreeMap::new();
        set.insert(u64_to_h160(1), ECRecover::execute as PrecompileFn);
        set.insert(u64_to_h160(2), Sha256::execute as PrecompileFn);
        set.insert(u64_to_h160(3), Ripemd160::execute as PrecompileFn);
        set.insert(u64_to_h160(4), Identity::execute as PrecompileFn);

        Precompiles { set }
    }
}

impl PrecompileSet for Precompiles {
    fn execute(
        &self,
        address: H160,
        input: &[u8],
        gas_limit: Option<u64>,
        context: &evm::Context,
        is_static: bool,
    ) -> Option<Result<PrecompileOutput, PrecompileFailure>> {
        if let Some(f) = self.set.get(&address) {
            Some(f(input, gas_limit, context, is_static))
        } else {
            None
        }
    }

    fn is_precompile(&self, address: H160) -> bool {
        self.set.contains_key(&address)
    }
}

fn u64_to_h160(index: u64) -> H160 {
    H160::from_low_u64_be(index)
}

