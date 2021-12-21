use std::num::TryFromIntError;

use ruc::RucError;

#[derive(Debug)]
pub enum Error {
    OverflowAdd,
    OverflowSub,
    MustBeNonConfidentialAssetAmount,
    KeyMustBeSet,
    TryFromIntError(core::num::TryFromIntError),
    BalanceNotEnough,
    LibfndoraError(libfindora::Error),
    JsonError(serde_json::Error),
    EthereumAddressFormatError,
    FraV1AddressFormatError,
    FraAddressFormatError,
    MnemonicFormatError,
    UnsupportMnemonicLanguage,
    RucError(Box<dyn RucError>),
    Bech32Error(bech32::Error),
    FromHexError(hex::FromHexError),
    Base64DecodeError(base64::DecodeError),
    Bip0039Error(bip0039::Error),
    ED25519BipError(ed25519_dalek_bip32::Error),
    DerivationPathError(derivation_path::DerivationPathError),
}

impl From<derivation_path::DerivationPathError> for Error {
    fn from(e: derivation_path::DerivationPathError) -> Self {
        Error::DerivationPathError(e)
    }
}

impl From<ed25519_dalek_bip32::Error> for Error {
    fn from(e: ed25519_dalek_bip32::Error) -> Self {
        Error::ED25519BipError(e)
    }
}

impl From<bip0039::Error> for Error {
    fn from(e: bip0039::Error) -> Self {
        Error::Bip0039Error(e)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error::Base64DecodeError(e)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(e: hex::FromHexError) -> Self {
        Error::FromHexError(e)
    }
}

impl From<bech32::Error> for Error {
    fn from(e: bech32::Error) -> Self {
        Error::Bech32Error(e)
    }
}

impl From<Box<dyn RucError>> for Error {
    fn from(e: Box<dyn RucError>) -> Self {
        Error::RucError(e)
    }
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

impl From<TryFromIntError> for Error {
    fn from(e: TryFromIntError) -> Self {
        Self::TryFromIntError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
