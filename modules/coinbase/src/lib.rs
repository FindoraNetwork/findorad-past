#![feature(generic_associated_types)]

mod module;
pub use module::CoinbaseModule;

mod utils;

mod error;
pub use error::{Error, Result};
