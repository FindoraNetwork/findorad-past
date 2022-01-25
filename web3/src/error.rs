use jsonrpc_core::{Error, ErrorCode};

pub fn no_impl() -> Error {
    Error {
        code: ErrorCode::ServerError(40001),
        message: String::from("No impl."),
        data: None,
    }
}
