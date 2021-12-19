#[derive(Debug)]
pub enum Error {
    OverflowAdd,
    RucError(Box<dyn ruc::RucError>),
    MustBeNonConfidentialAssetAmount,
    KeyMustBeSet,
}

impl From<Box<dyn ruc::RucError>> for Error {
    fn from(e: Box<dyn ruc::RucError>) -> Self {
        Self::RucError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
