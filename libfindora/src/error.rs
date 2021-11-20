use std::{array::TryFromSliceError, num::TryFromIntError};

use ruc::*;

pub fn convert_capnp_error(e: capnp::Error) -> abcf::Error {
    abcf::Error::ABCIApplicationError(80001, format!("{:?}", e))
}

pub fn convert_capnp_noinschema(e: capnp::NotInSchema) -> abcf::Error {
    abcf::Error::ABCIApplicationError(80002, format!("{:?}", e))
}

pub fn convert_ruc_error(e: Box<dyn RucError>) -> abcf::Error {
    abcf::Error::ABCIApplicationError(80003, format!("{:?}", e))
}

pub fn convert_try_slice_error(e: TryFromSliceError) -> abcf::Error {
    abcf::Error::ABCIApplicationError(80004, format!("{:?}", e))
}

pub fn convert_try_int_error(e: TryFromIntError) -> abcf::Error {
    abcf::Error::ABCIApplicationError(80004, format!("{:?}", e))
}

pub fn placeholder_error() -> abcf::Error {
    abcf::Error::ABCIApplicationError(80005, String::from("Only a placeholder"))
}
