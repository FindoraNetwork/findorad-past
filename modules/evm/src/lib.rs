#![feature(generic_associated_types)]

mod evm;
pub use crate::evm::EvmModule;

mod executor;
pub use executor::Executor;
use primitive_types::U256;

mod precompiles;

mod state;

pub const EVM_CHAIN_ID: U256 = U256([0, 0, 0, 418]);
