use std::num::TryFromIntError;

#[derive(Debug)]
pub enum Error {
    TryFromIntError(TryFromIntError),
    Bs3Error(abcf::bs3::Error),
    NoOutputIndex,
    OutputOperationMustBeEvm,
    RlpError(rlp::DecoderError),
    OnlySupportLegacyTransaction,
    Secp256k1Error(libsecp256k1::Error),
    AmountTypeMustBeNonConfidential,
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

impl From<libsecp256k1::Error> for Error {
    fn from(e: libsecp256k1::Error) -> Self {
        Self::Secp256k1Error(e)
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
            Error::RlpError(e) => abcf::Error::ABCIApplicationError(80005, format!("{:?}", e)),
            Error::OnlySupportLegacyTransaction => abcf::Error::ABCIApplicationError(
                80005,
                format!("Only support legact transaction."),
            ),
            Error::Secp256k1Error(e) => {
                abcf::Error::ABCIApplicationError(80005, format!("{:?}", e))
            }
            Error::AmountTypeMustBeNonConfidential => abcf::Error::ABCIApplicationError(
                80005,
                format!("Amount type must be non-confidential."),
            ),
        }
    }
}

impl From<rlp::DecoderError> for Error {
    fn from(e: rlp::DecoderError) -> Self {
        Self::RlpError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
