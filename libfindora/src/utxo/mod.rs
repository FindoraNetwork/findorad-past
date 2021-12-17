mod transaction;
pub use transaction::{Input, Output, UtxoTransaction};

pub type OutputId = Input;

mod validate;
pub use validate::ValidateTransaction;

mod rpc;
pub use rpc::{GetOwnedUtxoReq, GetOwnedUtxoResp, OwnedOutput};
