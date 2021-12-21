use std::fmt;

use crate::entry::wallet;

use console::{style, Emoji};

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
    typ: DisplayType,
    contents: Vec<Content>,
}

#[derive(Debug)]
enum DisplayType {
    ListWallet,
    Wallet,
    Address,
}

impl Display {
    fn fetcher(&self, p: &Option<String>) -> Result<String, fmt::Error> {
        match p {
            Some(v) => Ok(v.to_string()),
            None => Err(fmt::Error),
        }
    }

    fn empty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\n",
            Emoji("❓", ":("),
            style("There is no wallet exists").bold().red()
        )
    }

    fn list_wallet(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\n",
            Emoji("✨", ":)"),
            style("Listing").bold().green()
        )?;
        for (index, content) in self.contents.iter().enumerate() {
            let none = "[(none found)]".to_string();
            let name = content.name.as_ref().unwrap_or(&none);
            let address = self.fetcher(&content.address)?;
            write!(
                f,
                "
{} [{}]
name:       {}
address:    {}
                ",
                Emoji("👛", "$ "),
                index + 1,
                style(name).bold().white(),
                style(address).bold().white(),
            )?;
        }
        Ok(())
    }

    fn wallet(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.len() == 0 {
            return Err(fmt::Error);
        }

        let none = "[(none found)]".to_string();
        let name = &self.contents[0].name.as_ref().unwrap_or(&none);
        let mnemonic = self.fetcher(&self.contents[0].mnemonic)?;
        let address = self.fetcher(&self.contents[0].address)?;
        let public = self.fetcher(&self.contents[0].public)?;
        let secret = self.fetcher(&self.contents[0].secret)?;

        write!(
            f,
            "
{}
name:       {}
address:    {}
public:     {}
secret:     {}
mnemonic:   {}
            ",
            Emoji("👛", "$ "),
            style(name).bold().white(),
            style(address).bold().white(),
            style(public).bold().white(),
            style(secret).bold().white(),
            style(mnemonic).bold().white(),
        )
    }

    fn address(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.len() == 0 {
            return Err(fmt::Error);
        }

        let addr = self.fetcher(&self.contents[0].address)?;
        write!(
            f,
            "{} {}\n",
            Emoji("✨", ":)"),
            style("Success").bold().green()
        )?;
        write!(
            f,
            "{} Address: {}\n",
            Emoji("★ ", "* "),
            style(addr).white()
        )
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.len() == 0 {
            return self.empty(f);
        }

        match self.typ {
            DisplayType::ListWallet => self.list_wallet(f),
            DisplayType::Wallet => self.wallet(f),
            DisplayType::Address => self.address(f),
        }
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

        Display {
            contents,
            typ: DisplayType::ListWallet,
        }
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
            typ: DisplayType::Wallet,
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
            typ: DisplayType::Address,
        }
    }
}
