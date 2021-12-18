mod transaction;
pub use transaction::{Input, Output, Transaction};

pub type OutputId = Input;

mod validate;
pub use validate::ValidateTransaction;
