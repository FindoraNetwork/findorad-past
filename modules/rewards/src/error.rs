pub enum Error {
    WasmiError(wasmi::Error),
    VersionNoReturnValue,
    NoMemoryExport,
    ConvertIndexError,
    Bs3Error(abcf::bs3::Error),
}

impl From<wasmi::Error> for Error {
    fn from(e: wasmi::Error) -> Self {
        Self::WasmiError(e)
    }
}

impl From<abcf::bs3::Error> for Error {
    fn from(e: abcf::bs3::Error) -> Self {
        Error::Bs3Error(e)
    }
}

impl Error {
    pub fn to_abcf_error(self) -> abcf::Error {
        match self {
            Error::WasmiError(e) => abcf::Error,
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
