use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    OverflowAdd,
    OverflowSub,
    RucError(Box<dyn ruc::RucError>),
    MustBeNonConfidentialAssetAmount,
    KeyMustBeSet,
    TryFromIntError(core::num::TryFromIntError),
    BalanceNotEnough,
    LibfndoraError(libfindora::Error),
    JsonError(serde_json::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}

impl From<libfindora::Error> for Error {
    fn from(e: libfindora::Error) -> Self {
        Error::LibfndoraError(e)
    }
}

impl From<Box<dyn ruc::RucError>> for Error {
    fn from(e: Box<dyn ruc::RucError>) -> Self {
        Self::RucError(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromIntError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
