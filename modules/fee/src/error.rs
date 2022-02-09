#[derive(Debug)]
pub enum Error {
    MustBeNonConfidentialAmount,
    OverflowAdd,
    MustUseFraAndBlockHole,
    TryFromIntError(core::num::TryFromIntError),
}

impl From<core::num::TryFromIntError> for Error {
    fn from(e: core::num::TryFromIntError) -> Self {
        Error::TryFromIntError(e)
    }
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> abcf::Error {
        match e {
            Error::OverflowAdd => {
                abcf::Error::ABCIApplicationError(80005, String::from("Add overflow."))
            }
            Error::MustUseFraAndBlockHole => {
                abcf::Error::ABCIApplicationError(90002, String::from("Must use fra to pay fee."))
            }
            Error::MustBeNonConfidentialAmount => abcf::Error::ABCIApplicationError(
                90002,
                String::from("Must be non confidential amount."),
            ),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
