use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    TryFromIntError(TryFromIntError),
    Bs3Error(abcf::bs3::Error),
    NoOutputIndex,
    OutputOperationMustBeEvm,
}

impl From<abcf::bs3::Error> for Error {
    fn from(e: abcf::bs3::Error) -> Self {
        Self::Bs3Error(e)
    }
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
            Error::Bs3Error(e) => abcf::Error::ABCIApplicationError(80005, format!("{:?}", e)),
            Error::NoOutputIndex => abcf::Error::ABCIApplicationError(80005, format!("No output.")),
            Error::OutputOperationMustBeEvm => abcf::Error::ABCIApplicationError(
                80005,
                format!("Output operation must be evm call."),
            ),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
