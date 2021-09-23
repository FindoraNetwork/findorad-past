mod transaction;
pub use transaction::{Input, Input as OutputId, UtxoTransacrion};

mod validate;
pub use validate::ValidateTransaction;

mod rpc;
pub use rpc::{GetOwnedUtxoReq, GetOwnedUtxoResp, OwnedOutput};
