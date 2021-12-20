use std::num::TryFromIntError;

use libfindora::asset::Amount;

#[derive(Debug)]
pub enum Error {
    OverflowAdd,
    OverflowMul,
    MustUseFraAndBlockHole,
    MustBeNonConfidentialAmount,
    MustBeFra,
    Bs3Error(abcf::bs3::Error),
    DelegateAmountOutOfRange(Amount, Amount),
    AlreadySelfDelegate,
    TryFromIntError(TryFromIntError),
    MustDoSelfDegegateFirst,
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> abcf::Error {
        match e {
            Error::OverflowAdd => {
                abcf::Error::ABCIApplicationError(80005, String::from("Add overflow."))
            }
            Error::OverflowMul => {
                abcf::Error::ABCIApplicationError(80005, String::from("Mul overflow."))
            }
            Error::MustUseFraAndBlockHole => {
                abcf::Error::ABCIApplicationError(90002, String::from("Must use fra to pay fee."))
            }
            Error::MustBeNonConfidentialAmount => abcf::Error::ABCIApplicationError(
                90002,
                String::from("Must be non confidential amount."),
            ),
            Error::MustBeFra => {
                abcf::Error::ABCIApplicationError(90002, String::from("AssetType must be fra."))
            }
            Error::Bs3Error(e) => abcf::Error::ABCIApplicationError(90002, format!("{:?}", e)),
            Error::DelegateAmountOutOfRange(min, max) => abcf::Error::ABCIApplicationError(
                90002,
                format!("Delegate amount out of range, [min: {}, max: {}]", min, max),
            ),
            Error::AlreadySelfDelegate => abcf::Error::ABCIApplicationError(
                90002,
                format!("validator already do self-delegate."),
            ),
            Error::TryFromIntError(e) => {
                abcf::Error::ABCIApplicationError(90002, format!("{:?}", e))
            }
            Error::MustDoSelfDegegateFirst => {
                abcf::Error::ABCIApplicationError(90002, format!("Must do self delegate."))
            }
        }
    }
}

impl From<abcf::bs3::Error> for Error {
    fn from(e: abcf::bs3::Error) -> Self {
        Error::Bs3Error(e)
    }
}

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Error::TryFromIntError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
