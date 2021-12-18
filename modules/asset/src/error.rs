use libfindora::{asset::AssetType, Address};

#[derive(Debug)]
pub enum Error {
    Bs3Error(abcf::bs3::Error),
    AssetTypeAlreadyExists(AssetType),
    AssetTypeNotExists(AssetType),
    MustBeNonConfidentialAmount,
    IssueMustBeOwner(Address, Address),
    AssetCantTransfer(AssetType),
    Unknown,
}

impl From<Error> for abcf::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::AssetTypeAlreadyExists(e) => {
                abcf::Error::ABCIApplicationError(90001, format!("asset {:?} already exists", e))
            }
            Error::MustBeNonConfidentialAmount => abcf::Error::ABCIApplicationError(
                90002,
                String::from("must be nonconfidential asset"),
            ),
            Error::Bs3Error(e) => e.into(),
            Error::IssueMustBeOwner(issuer, owner) => abcf::Error::ABCIApplicationError(
                90003,
                format!(
                    "only owner {:?} can issue this asset, provide is {:?}",
                    owner, issuer
                ),
            ),
            Error::AssetTypeNotExists(e) => {
                abcf::Error::ABCIApplicationError(90004, format!("asset {:?} not exists", e))
            }
            Error::AssetCantTransfer(e) => {
                abcf::Error::ABCIApplicationError(90005, format!("asset {:?} can't transfer", e))
            }
            Error::Unknown => {
                abcf::Error::ABCIApplicationError(100001, String::from("placeholder error"))
            }
        }
    }
}

impl From<abcf::bs3::Error> for Error {
    fn from(e: abcf::bs3::Error) -> Self {
        Self::Bs3Error(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
