use evm::executor::stack::{PrecompileFailure, PrecompileOutput, PrecompileSet};
use primitive_types::H160;

pub struct PreCompiles {}

impl PrecompileSet for PreCompiles {
    fn is_precompile(&self, _address: H160) -> bool {
        false
    }

    fn execute(
        &self,
        _address: H160,
        _input: &[u8],
        _gas_limit: Option<u64>,
        _context: &evm::Context,
        _is_static: bool,
    ) -> Option<Result<PrecompileOutput, PrecompileFailure>> {
        None
    }
}
