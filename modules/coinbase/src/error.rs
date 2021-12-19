use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    TryFromIntError(TryFromIntError),
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromIntError(e)
    }
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> abcf::Error {
        match e {
            Error::TryFromIntError(e) => {
                abcf::Error::ABCIApplicationError(80005, format!("{:?}", e))
            }
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
