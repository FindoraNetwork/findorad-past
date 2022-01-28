#[derive(Debug)]
pub enum Error {
    AbcfError(abcf::Error),
    SledError(sled::Error),
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        Error::SledError(e)
    }
}

impl From<abcf::Error> for Error {
    fn from(e: abcf::Error) -> Self {
        Error::AbcfError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
