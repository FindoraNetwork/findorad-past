#![feature(generic_associated_types)]

mod module;
pub use module::AssetModule;

mod error;
pub use error::{Error, Result};

mod utils;

mod transaction;
pub use transaction::{AssetInfo, AssetIssue, Transaction};
