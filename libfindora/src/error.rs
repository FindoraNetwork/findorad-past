use std::{array::TryFromSliceError, num::TryFromIntError};

// pub fn convert_capnp_error(e: capnp::Error) -> abcf::Error {
//     abcf::Error::ABCIApplicationError(80001, format!("{:?}", e))
// }
//
// pub fn convert_capnp_noinschema(e: capnp::NotInSchema) -> abcf::Error {
//     abcf::Error::ABCIApplicationError(80002, format!("{:?}", e))
// }
//
// pub fn convert_ruc_error(e: Box<dyn ruc::RucError>) -> abcf::Error {
//     abcf::Error::ABCIApplicationError(80003, format!("{:?}", e))
// }
//
// pub fn convert_try_slice_error(e: TryFromSliceError) -> abcf::Error {
//     abcf::Error::ABCIApplicationError(80004, format!("{:?}", e))
// }
//
// pub fn convert_try_int_error(e: TryFromIntError) -> abcf::Error {
//     abcf::Error::ABCIApplicationError(80004, format!("{:?}", e))
// }
//
// pub fn placeholder_error() -> abcf::Error {
//     abcf::Error::ABCIApplicationError(80005, String::from("Only a placeholder"))
// }
//
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CapnpError(capnp::Error),
    CapnpNotInSchema(capnp::NotInSchema),
    RucError(Box<dyn ruc::RucError>),
    TryFromSliceError(TryFromSliceError),
    TryFromIntError(TryFromIntError),
    ChaumPedersenProofParseError,
    Unknown,
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
                abcf::Error::ABCIApplicationError(90002, format!("{:?}", e))
            }
            Error::RucError(e) => abcf::Error::ABCIApplicationError(90003, format!("{:?}", e)),
            Error::TryFromSliceError(e) => {
                abcf::Error::ABCIApplicationError(90004, format!("{:?}", e))
            }
            Error::TryFromIntError(e) => {
                abcf::Error::ABCIApplicationError(90005, format!("{:?}", e))
            }
            Error::ChaumPedersenProofParseError => abcf::Error::ABCIApplicationError(
                90006,
                String::from("parse error, chaum_pedersen_proof_x must have 1 or 2 proof."),
            ),
            Error::Unknown => {
                abcf::Error::ABCIApplicationError(100001, String::from("Only placeholder"))
            }
        }
    }
}
