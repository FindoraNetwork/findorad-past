use std::{array::TryFromSliceError, num::TryFromIntError};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CapnpError(capnp::Error),
    CapnpNotInSchema(capnp::NotInSchema),
    RucError(Box<dyn ruc::RucError>),
    TryFromSliceError(TryFromSliceError),
    TryFromIntError(TryFromIntError),
    ChaumPedersenProofParseError,
    OverflowAdd,
    Unknown,
    MustBeNonConfidentialAsset,
    AlreadySign,
}

impl From<capnp::Error> for Error {
    fn from(e: capnp::Error) -> Self {
        Self::CapnpError(e)
    }
}

impl From<capnp::NotInSchema> for Error {
    fn from(e: capnp::NotInSchema) -> Self {
        Self::CapnpNotInSchema(e)
    }
}

impl From<Box<dyn ruc::RucError>> for Error {
    fn from(e: Box<dyn ruc::RucError>) -> Self {
        Self::RucError(e)
    }
}

impl From<TryFromSliceError> for Error {
    fn from(e: TryFromSliceError) -> Self {
        Self::TryFromSliceError(e)
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
            Error::CapnpError(e) => abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)),
            Error::CapnpNotInSchema(e) => {
                abcf::Error::ABCIApplicationError(80002, format!("{:?}", e))
            }
            Error::RucError(e) => abcf::Error::ABCIApplicationError(90003, format!("{:?}", e)),
            Error::TryFromSliceError(e) => {
                abcf::Error::ABCIApplicationError(80004, format!("{:?}", e))
            }
            Error::TryFromIntError(e) => {
                abcf::Error::ABCIApplicationError(80005, format!("{:?}", e))
            }
            Error::ChaumPedersenProofParseError => abcf::Error::ABCIApplicationError(
                90006,
                String::from("parse error, chaum_pedersen_proof_x must have 1 or 2 proof."),
            ),
            Error::OverflowAdd => {
                abcf::Error::ABCIApplicationError(80007, String::from("add overflow"))
            }
            Error::MustBeNonConfidentialAsset => abcf::Error::ABCIApplicationError(
                80008,
                String::from("mustbe nonconfidential asset type."),
            ),
            Error::AlreadySign => {
                abcf::Error::ABCIApplicationError(80009, String::from("already sign"))
            }
            Error::Unknown => {
                abcf::Error::ABCIApplicationError(81000, String::from("Only placeholder"))
            }
        }
    }
}
