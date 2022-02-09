use std::collections::BTreeMap;

use evm::executor::stack::{PrecompileFailure, PrecompileFn, PrecompileOutput, PrecompileSet};
use precompile_blake2::Blake2F;
use precompile_bn128::{Bn128Add, Bn128Mul, Bn128Pairing};
use precompile_core::Precompile;
use precompile_curve25519::{Curve25519Add, Curve25519ScalarMul};
use precompile_ed25519::Ed25519Verify;
use precompile_modexp::Modexp;
use precompile_sha3fips::Sha3FIPS256;
use precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use primitive_types::H160;

#[derive(Default)]
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
        set.insert(u64_to_h160(5), Modexp::execute as PrecompileFn);
        set.insert(u64_to_h160(6), ECRecoverPublicKey::execute as PrecompileFn);
        set.insert(u64_to_h160(7), Sha3FIPS256::execute as PrecompileFn);
        set.insert(u64_to_h160(1024), Blake2F::execute as PrecompileFn);
        set.insert(u64_to_h160(1025), Bn128Pairing::execute as PrecompileFn);
        set.insert(u64_to_h160(1026), Bn128Add::execute as PrecompileFn);
        set.insert(u64_to_h160(1027), Bn128Mul::execute as PrecompileFn);
        set.insert(u64_to_h160(1028), Curve25519Add::execute as PrecompileFn);
        set.insert(
            u64_to_h160(1029),
            Curve25519ScalarMul::execute as PrecompileFn,
        );
        set.insert(u64_to_h160(1030), Ed25519Verify::execute as PrecompileFn);

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
