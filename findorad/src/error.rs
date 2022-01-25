#[derive(Debug)]
pub enum Error {
    AbcfError(abcf::Error),
}

impl From<abcf::Error> for Error {
    fn from(e: abcf::Error) -> Self {
        Error::AbcfError(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
