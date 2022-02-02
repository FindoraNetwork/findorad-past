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
    pub fn to_rpc_error(self) -> abcf::Error {
        match self {
            Error::WasmiError(e) => abcf::Error::RPCApplicationError(90001, format!("{:?}", e)),
            Error::Bs3Error(e) => abcf::Error::RPCApplicationError(90002, format!("{:?}", e)),
            Error::VersionNoReturnValue => abcf::Error::RPCApplicationError(90003, String::from("version return no value")),
            Error::NoMemoryExport => abcf::Error::RPCApplicationError(90004, String::from("No exported memory")),
            Error::ConvertIndexError => abcf::Error::RPCApplicationError(90005, String::from("convert index error")),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
