use jsonrpc_core::{Error, ErrorCode};

pub fn no_impl() -> Error {
    Error {
        code: ErrorCode::ServerError(40001),
        message: String::from("No impl."),
        data: None,
    }
}

pub fn sdk_error(e: abcf_sdk::error::Error) -> Error {
    Error {
        code: ErrorCode::ServerError(40002),
        message: format!("{:?}", e),
        data: None,
    }
}

pub fn empty_reponse() -> Error {
    Error {
        code: ErrorCode::ServerError(40003),
        message: "upsteam return null response".to_string(),
        data: None,
    }
}

pub fn convert_error(e: std::num::TryFromIntError) -> Error {
    Error {
        code: ErrorCode::ServerError(40004),
        message: format!("{:?}", e),
        data: None,
    }
}

