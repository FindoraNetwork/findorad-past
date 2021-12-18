#![feature(generic_associated_types)]

mod module;
pub use module::CoinbaseModule;

mod error;
pub use error::{Error, Result};
