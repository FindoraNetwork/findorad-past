use std::fmt;

use crate::entry::asset;

use console::{style, Emoji};

#[derive(Default, Debug)]
pub struct Content {
    pub code: Option<String>,
    pub address: Option<String>,
    pub asset_type: Option<String>,
    pub memo: Option<String>,
    pub maximun: Option<String>,
    pub amount: Option<String>,
    pub is_transferable: Option<String>,
    pub is_issued: Option<String>,
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
    Issue,
}

impl Display {
    pub fn new(typ: DisplayType, assets: Vec<(asset::Asset, Option<&u64>)>) -> Display {
        let contents = assets
            .iter()
            .map(|a| Content {
                code: a.0.code.clone(),
                address: Some(a.0.address.clone()),
                asset_type: Some(a.0.get_asset_type_base64()),
                memo: a.0.memo.clone(),
                maximun: a.0.maximun.map(|u| u.to_string()),
                amount: a.1.map(|u| u.to_string()),
                is_transferable: Some(a.0.is_transferable.to_string()),
                is_issued: Some(a.0.is_issued.to_string()),
            })
            .collect();

        Display { typ, contents }
    }

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
            style("There is no asset exists").bold().red()
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
            let none = "[(none found)]".to_string();
            let code = self.contents[index].code.as_ref().unwrap_or(&none);
            let eth_compatible_address = self.fetcher(&self.contents[index].address)?;
            let asset_type = self.fetcher(&self.contents[index].asset_type)?;
            write!(
                f,
                "
{} [{}]
Code:                   {}
ETH Compatible Address: {}
Asset Type:             {}
                ",
                Emoji("🪙", "$ "),
                index + 1,
                style(code).bold().white(),
                style(eth_compatible_address).bold().white(),
                style(asset_type).bold().white(),
            )?;
        }
        Ok(())
    }

    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return Err(fmt::Error);
        }

        let none = "[(none found)]".to_string();
        let code = &self.contents[0].code.as_ref().unwrap_or(&none);
        let address = self.fetcher(&self.contents[0].address)?;
        let memo = &self.contents[0].memo.as_ref().unwrap_or(&none);
        let maximun = &self.contents[0].maximun.as_ref().unwrap_or(&none);
        let amount = self.fetcher(&self.contents[0].amount)?;
        let is_transferable = &self.contents[0].is_transferable.as_ref().unwrap_or(&none);
        let is_issued = &self.contents[0].is_issued.as_ref().unwrap_or(&none);

        write!(
            f,
            "
{}
Code:                   {}
ETH Compatible Address: {}
Memo:                   {}
Maximun:                {}
Amount:                 {}
Is Transferable:        {}
Is Issued:              {}
            ",
            Emoji("🪙", "$ "),
            style(code).bold().cyan(),
            style(address).bold().cyan(),
            style(memo).bold().cyan(),
            style(maximun).bold().cyan(),
            style(amount).bold().cyan(),
            style(is_transferable).bold().magenta(),
            style(is_issued).bold().magenta(),
        )
    }

    fn create(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return Err(fmt::Error);
        }

        let address = self.fetcher(&self.contents[0].address)?;
        let asset_type = self.fetcher(&self.contents[0].asset_type)?;
        write!(
            f,
            "
{} {}
{} ETH Compatible Address: {} 
{} Asset Type:             {}
",
            Emoji("✨", ":)"),
            style("Success Created").bold().green(),
            Emoji("★ ", "* "),
            style(address).white(),
            Emoji("★ ", "* "),
            style(asset_type).white()
        )
    }

    fn issue(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return Err(fmt::Error);
        }

        let address = self.fetcher(&self.contents[0].address)?;
        let asset_type = self.fetcher(&self.contents[0].asset_type)?;
        let code = self.fetcher(&self.contents[0].code)?;
        write!(
            f,
            "
{} {}
{} ETH Compatible Address: {}
{} Asset Type:             {}
{} Code:                   {}
",
            Emoji("✨", ":)"),
            style("Success Issued").bold().green(),
            Emoji("★ ", "* "),
            style(address).white(),
            Emoji("★ ", "* "),
            style(asset_type).white(),
            Emoji("★ ", "* "),
            style(code).white(),
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
            DisplayType::Create => self.create(f),
            DisplayType::Issue => self.issue(f),
        }
    }
}
