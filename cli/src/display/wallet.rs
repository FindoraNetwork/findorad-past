use std::fmt;

use crate::entry::wallet;

#[derive(Default, Debug)]
pub struct Content {
    name: Option<String>,
    mnemonic: Option<String>,
    address: Option<String>,
    public: Option<String>,
    secret: Option<String>,
}

#[derive(Debug)]
pub struct Display {
    contents: Vec<Content>,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:#?})", self.contents)
    }
}

impl From<Vec<wallet::ListWallet>> for Display {
    fn from(w: Vec<wallet::ListWallet>) -> Display {
        let contents = w
            .iter()
            .map(|v| Content {
                name: v.name.clone(),
                address: Some(v.address.clone()),
                ..Default::default()
            })
            .collect();

        Display { contents }
    }
}

impl From<wallet::Wallet> for Display {
    fn from(w: wallet::Wallet) -> Display {
        Display {
            contents: vec![Content {
                name: w.name,
                mnemonic: Some(w.mnemonic),
                address: Some(w.address),
                public: Some(w.public),
                secret: Some(w.secret),
            }],
        }
    }
}

impl From<String> for Display {
    fn from(w: String) -> Display {
        Display {
            contents: vec![Content {
                address: Some(w),
                ..Default::default()
            }],
        }
    }
}
