use evm::{
    backend::Backend,
    executor::stack::{MemoryStackState, StackExecutor, StackSubstateMetadata},
    Config, ExitReason,
};
use primitive_types::U256;

use crate::{precompile::Precompiles, rpc::CallRequest};

fn _call(
    tx: CallRequest,
    backend: &impl Backend,
    config: &Config,
    precompiles: &Precompiles,
) -> (ExitReason, Vec<u8>, u64) {
    let metadata = StackSubstateMetadata::new(tx.gas_limit, config);

    let stack = MemoryStackState::new(metadata, backend);

    let mut executor = StackExecutor::new_with_precompiles(stack, config, precompiles);

    let res = executor.transact_call(
        tx.from.0,
        tx.to.0,
        U256::from(tx.amount),
        tx.data,
        tx.gas_limit,
        Vec::new(),
    );

    let gas_price = executor.used_gas();

    (res.0, res.1, gas_price)
}

pub fn call(
    tx: CallRequest,
    backend: &impl Backend,
    precompiles: &Precompiles,
) -> (ExitReason, Vec<u8>, u64) {
    let config = Config::istanbul();

    _call(tx, backend, &config, precompiles)
}

pub fn estimate_gas(
    tx: CallRequest,
    backend: &impl Backend,
    precompiles: &Precompiles,
) -> (ExitReason, Vec<u8>, u64) {
    let mut config = Config::istanbul();

    config.estimate = true;

    _call(tx, backend, &config, precompiles)
}
