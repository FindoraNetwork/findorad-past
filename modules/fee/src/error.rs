#[derive(Debug)]
pub enum Error {
    OverflowAdd,
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> abcf::Error {
        match e {
            Error::OverflowAdd => {
                abcf::Error::ABCIApplicationError(80005, String::from("Add overflow."))
            }
        }
    }
}

pub type Result<T> = core::result::Result<T, Error>;
