#![feature(generic_associated_types)]

mod transaction;

mod module;
pub use module::RewardsModule;

mod runtime;

pub mod rpc;

mod error;
pub use error::*;
