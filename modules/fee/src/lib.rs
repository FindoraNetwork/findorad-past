#![feature(generic_associated_types)]

mod module;
pub use module::FeeModule;

mod transaction;
pub use transaction::{Transaction, FRA_FEE_AMOUNT};

mod error;
pub use error::{Error, Result};
