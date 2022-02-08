use libfindora::utxo::OutputId;

#[derive(Debug)]
pub enum Error {
    NoUnspentOutput(OutputId),
    UtxoBalanceError(String),
    Bs3Error(abcf::bs3::Error),
    TryFromIntError(core::num::TryFromIntError),
    DuplicateOutput(OutputId),
    MissingOutput(OutputId),
    RucError(Box<dyn ruc::RucError>),
    AddOverflow,
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
            Error::RucError(e) => Self::ABCIApplicationError(90003, format!("{:?}", e)),
            Error::AddOverflow => {
                abcf::Error::ABCIApplicationError(80005, String::from("Add overflow."))
            }
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

impl From<Box<dyn ruc::RucError>> for Error {
    fn from(e: Box<dyn ruc::RucError>) -> Self {
        Self::RucError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
