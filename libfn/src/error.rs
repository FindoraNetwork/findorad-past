use std::{num::TryFromIntError, fmt::Display};

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

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Self::OverflowSub => "add overflow",
            Self::OverflowAdd => "sub overflow",
            Self::MustBeNonConfidentialAssetAmount => "must be use non-confidential asset and non-confidential amount",
            Self::KeyMustBeSet => "public key must be set.",
            Self::TryFromIntError(_) => "convert int error",
            Self::BalanceNotEnough => "balance not enough",
            Self::LibfndoraError(_) => "libfindora error",
            Self::JsonError(_) => "json convert error",
            Self::EthereumAddressFormatError => "ethereum address format error.",
            Self::FraV1AddressFormatError => "fra v1 address format error",
            Self::FraAddressFormatError => "fra address format error",
            Self::MnemonicFormatError => "mnemonic format error",
            Self::UnsupportMnemonicLanguage => "unsupport mnemonic language",
            Self::RucError(_) => "error cause by ruc",
            Self::Bech32Error(_) => "bech32 error",
            Self::FromHexError(_) => "from hex error",
            Self::Base64DecodeError(_) => "base64 decode error",
            Self::Bip0039Error(_) => "bip0039 error",
            Self::ED25519BipError(_) => "ed25519 bip error",
            Self::DerivationPathError(_) => "derivation_path error",
        }
    }
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
