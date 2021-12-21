use std::fmt;

use crate::entry::wallet;

use console::{style, Emoji};

#[derive(Default, Debug)]
pub struct Content {
    pub name: Option<String>,
    pub eth_compatible_address: Option<String>,
    pub fra_address: Option<String>,
    pub public_key: Option<String>,
    pub secret: Option<String>,
    pub mnemonic: Option<String>,
}

#[derive(Debug)]
pub struct Display {
    typ: DisplayType,
    contents: Vec<Content>,
}

#[derive(Debug)]
pub enum DisplayType {
    List,
    Show,
    Create,
    Delete,
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

    fn list(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}\n",
            Emoji("✨", ":)"),
            style("Listing").bold().green()
        )?;
        for (index, content) in self.contents.iter().enumerate() {
            let none = "[(none found)]".to_string();
            let name = content.name.as_ref().unwrap_or(&none);
            let eth_compatible_address = self.fetcher(&self.contents[0].eth_compatible_address)?;
            write!(
                f,
                "
{} [{}]
Name:                   {}
ETH Compatible Address: {}
                ",
                Emoji("👛", "$ "),
                index + 1,
                style(name).bold().white(),
                style(eth_compatible_address).bold().white(),
            )?;
        }
        Ok(())
    }

    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.len() == 0 {
            return Err(fmt::Error);
        }

        let none = "[(none found)]".to_string();
        let name = &self.contents[0].name.as_ref().unwrap_or(&none);
        let eth_compatible_address = self.fetcher(&self.contents[0].eth_compatible_address)?;
        let fra_address = self.fetcher(&self.contents[0].fra_address)?;
        let public_key = self.fetcher(&self.contents[0].public_key)?;
        let secret = self.fetcher(&self.contents[0].secret)?;
        let mnemonic = self.fetcher(&self.contents[0].mnemonic)?;

        write!(
            f,
            "
{}
Name:                   {}
ETH Compatible Address: {}
Findora Address:        {}
Findora Public key:     {}
Secret:                 {}
Mnemonic: {}
            ",
            Emoji("👛", "$ "),
            style(name).bold().white(),
            style(eth_compatible_address).bold().white(),
            style(fra_address).bold().white(),
            style(public_key).bold().white(),
            style(secret).bold().white(),
            style(mnemonic).bold().white(),
        )
    }

    fn create(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.len() == 0 {
            return Err(fmt::Error);
        }

        let addr = self.fetcher(&self.contents[0].eth_compatible_address)?;
        write!(
            f,
            "{} {}\n",
            Emoji("✨", ":)"),
            style("Success Created").bold().green()
        )?;
        write!(
            f,
            "{} ETH Compatible Address: {}\n",
            Emoji("★ ", "* "),
            style(addr).white()
        )
    }

    fn delete(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.len() == 0 {
            return Err(fmt::Error);
        }

        let addr = self.fetcher(&self.contents[0].eth_compatible_address)?;
        write!(
            f,
            "{} {}\n",
            Emoji("✨", ":)"),
            style("Success Deleted").bold().green()
        )?;
        write!(
            f,
            "{} ETH Compatible Address: {}\n",
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
            DisplayType::List => self.list(f),
            DisplayType::Show => self.show(f),
            DisplayType::Create => self.create(f),
            DisplayType::Delete => self.delete(f),
        }
    }
}

impl From<(String, DisplayType)> for Display {
    fn from(w: (String, DisplayType)) -> Display {
        Display {
            contents: vec![Content {
                eth_compatible_address: Some(w.0),
                ..Default::default()
            }],
            typ: w.1,
        }
    }
}

impl From<Vec<wallet::ListWallet>> for Display {
    fn from(w: Vec<wallet::ListWallet>) -> Display {
        let contents = w
            .iter()
            .map(|v| Content {
                name: v.name.clone(),
                eth_compatible_address: Some(v.address.clone()),
                ..Default::default()
            })
            .collect();

        Display {
            contents,
            typ: DisplayType::List,
        }
    }
}

impl From<Content> for Display {
    fn from(c: Content) -> Display {
        Display {
            contents: vec![c],
            typ: DisplayType::Show,
        }
    }
}
