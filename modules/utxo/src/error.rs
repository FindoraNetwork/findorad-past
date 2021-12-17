use libfindora::utxo::OutputId;

#[derive(Debug)]
pub enum Error {
    NoUnspentOutput(OutputId),
    UtxoBalanceError(String),
    Bs3Error(abcf::bs3::Error),
    TryFromIntError(core::num::TryFromIntError),
    DuplicateOutput(OutputId),
    MissingOutput(OutputId),
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::NoUnspentOutput(i) => {
                Self::ABCIApplicationError(90001, format!("Can't find unspent output: {:?}", i))
            }
            Error::DuplicateOutput(i) => {
                Self::ABCIApplicationError(90001, format!("Output already exists: {:?}", i))
            }
            Error::MissingOutput(i) => {
                Self::ABCIApplicationError(90001, format!("Output does't exists: {:?}", i))
            }
            Error::UtxoBalanceError(i) => Self::ABCIApplicationError(90002, i),
            Error::Bs3Error(e) => e.into(),
            Error::TryFromIntError(e) => Self::ABCIApplicationError(90003, format!("{:?}", e)),
        }
    }
}

impl From<abcf::bs3::Error> for Error {
    fn from(e: abcf::bs3::Error) -> Self {
        Self::Bs3Error(e)
    }
}

impl From<core::num::TryFromIntError> for Error {
    fn from(e: core::num::TryFromIntError) -> Self {
        Self::TryFromIntError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
