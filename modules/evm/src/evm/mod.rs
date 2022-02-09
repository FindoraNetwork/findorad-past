use evm::{backend::Backend, executor::stack::MemoryStackState};

use crate::transaction::EvmTransaction;

pub mod account;
pub mod backend;
pub mod transfer;
pub mod vicinity;

pub fn run_tx<'backend, 'config>(
    _tx: EvmTransaction,
    stack: MemoryStackState<'backend, 'config, impl Backend>,
) -> MemoryStackState<'backend, 'config, impl Backend> {
    stack
}
