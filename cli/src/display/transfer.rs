use std::fmt;

use console::{style, Emoji};

#[derive(Default, Debug)]
pub struct Content {
    pub name: Option<String>,
    pub contains_len: Option<String>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub public_key: Option<String>,
    pub amount: Option<String>,
    pub asset_type: Option<String>,
    pub is_confidential_amount: Option<String>,
    pub is_confidential_asset: Option<String>,
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
    Save,
    Send,
    Batch,
}

impl Display {
    fn fetcher(&self, p: &Option<String>) -> Result<String, fmt::Error> {
        match p {
            Some(v) => Ok(v.to_string()),
            None => Err(fmt::Error),
        }
    }

    fn empty(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            Emoji("❓", ":("),
            style("There is no transfer records").bold().red()
        )
    }

    fn list(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{} {}",
            Emoji("✨", ":)"),
            style("Listing").bold().green()
        )?;
        for index in 0..self.contents.len() {
            let name = self.fetcher(&self.contents[index].name)?;
            let contains_len = self.fetcher(&self.contents[index].contains_len)?;
            write!(
                f,
                "
{} [{}]
Batch Name: {} ({})
                ",
                Emoji("🚚", "<<"),
                index + 1,
                style(name).bold().white(),
                style(contains_len).bold().white(),
            )?;
        }
        Ok(())
    }

    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for index in 0..self.contents.len() {
            let name = self.fetcher(&self.contents[index].name)?;
            let from_address = self.fetcher(&self.contents[index].from_address)?;
            let to_address = self.fetcher(&self.contents[index].to_address)?;
            let public_key = self.fetcher(&self.contents[index].public_key)?;
            let amount = self.fetcher(&self.contents[index].amount)?;
            let asset_type = self.fetcher(&self.contents[index].asset_type)?;
            let is_confidential_amount =
                self.fetcher(&self.contents[index].is_confidential_amount)?;
            let is_confidential_asset =
                self.fetcher(&self.contents[index].is_confidential_asset)?;
            write!(
                f,
                "
{}
Batch Name:             {}
From Address:           {}
To Address:             {}
Amount:                 {}
Public Key:             {}
Asset Type:             {}
Is Confidential Amount: {}
Is Confidential Asset:  {}
",
                Emoji("📦", "P "),
                style(name).bold().white(),
                style(from_address).bold().white(),
                style(to_address).bold().white(),
                style(amount).bold().white(),
                style(public_key).bold().white(),
                style(asset_type).bold().white(),
                style(is_confidential_amount).bold().white(),
                style(is_confidential_asset).bold().white(),
            )?;
        }
        Ok(())
    }

    fn save(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.fetcher(&self.contents[0].name)?;
        write!(
            f,
            "
{} {}
{} Batch Name: {}
",
            Emoji("✨", ":)"),
            style("Success Saved").bold().green(),
            Emoji("★ ", "* "),
            style(name).white()
        )
    }

    fn send(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let from_address = self.fetcher(&self.contents[0].from_address)?;
        let to_address = self.fetcher(&self.contents[0].to_address)?;
        let amount = self.fetcher(&self.contents[0].amount)?;
        write!(
            f,
            "
{} {}
From:   {} 
To:     {} 
Amount: {}
",
            Emoji("✨", ":)"),
            style("Success Sent").bold().green(),
            style(from_address).white(),
            style(to_address).white(),
            style(amount).white(),
        )
    }

    fn batch(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self.fetcher(&self.contents[0].name)?;
        write!(
            f,
            "
{} {}
{} Batch Name: {}
",
            Emoji("✨", ":)"),
            style("Success Batch Sent").bold().green(),
            Emoji("★ ", "* "),
            style(name).white()
        )
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return self.empty(f);
        }

        match self.typ {
            DisplayType::List => self.list(f),
            DisplayType::Show => self.show(f),
            DisplayType::Save => self.save(f),
            DisplayType::Send => self.send(f),
            DisplayType::Batch => self.batch(f),
        }
    }
}

impl From<Vec<(String, usize)>> for Display {
    fn from(w: Vec<(String, usize)>) -> Display {
        let contents = w
            .into_iter()
            .map(|v| Content {
                name: Some(v.0),
                contains_len: Some(v.1.to_string()),
                ..Default::default()
            })
            .collect();

        Display {
            contents,
            typ: DisplayType::List,
        }
    }
}

impl From<(String, DisplayType)> for Display {
    fn from(w: (String, DisplayType)) -> Display {
        Display {
            contents: vec![Content {
                name: Some(w.0),
                ..Default::default()
            }],
            typ: w.1,
        }
    }
}

impl From<(String, String, u64)> for Display {
    fn from(w: (String, String, u64)) -> Display {
        Display {
            contents: vec![Content {
                from_address: Some(w.0),
                to_address: Some(w.1),
                amount: Some(w.2.to_string()),
                ..Default::default()
            }],
            typ: DisplayType::Send,
        }
    }
}

impl From<Vec<Content>> for Display {
    fn from(contents: Vec<Content>) -> Display {
        Display {
            contents,
            typ: DisplayType::Show,
        }
    }
}
