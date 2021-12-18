use libfindora::utxo::OutputId;

#[derive(Debug)]
pub enum Error {
    Unknown,
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::Unknown => Self::ABCIApplicationError(100001, String::from("placeholder error")),
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
