use std::fmt;

use crate::entry::asset;

use console::{style, Emoji};

#[derive(Default, Debug)]
pub struct Content {
    pub name: Option<String>,
    pub address: Option<String>,
    pub asset_type: Option<String>,
    pub memo: Option<String>,
    pub maximun: Option<String>,
    pub amount: Option<String>,
    pub is_transferable: Option<String>,
    pub is_issued: Option<String>,
    pub is_confidential_amount: Option<String>,
}

#[derive(Debug)]
pub struct Display {
    typ: DisplayType,
    contents: Vec<Content>,
}

#[derive(Debug)]
pub enum DisplayType {
    Show,
    Create,
    Issue,
}

impl Display {
    pub fn new(typ: DisplayType, assets: Vec<(asset::Asset, Option<u64>)>) -> Display {
        let contents = assets
            .iter()
            .map(|a| Content {
                name: a.0.name.clone(),
                address: Some(a.0.address.clone()),
                asset_type: Some(a.0.get_asset_type_base64()),
                memo: a.0.memo.clone(),
                maximun: a
                    .0
                    .maximum
                    .map(|u| ((u as f64) / (10_f64.powf(a.0.decimal_place as f64))).to_string()),
                amount: a.1.map(|u| {
                    if a.0.is_confidential_amount {
                        format!(
                            "{} (Confidential)",
                            ((u as f64) / (10_f64.powf(a.0.decimal_place as f64)))
                        )
                    } else {
                        format!(
                            "{} (Nonconfidential)",
                            ((u as f64) / (10_f64.powf(a.0.decimal_place as f64)))
                        )
                    }
                }),
                is_transferable: Some(a.0.is_transferable.to_string()),
                is_issued: Some(a.0.is_issued.to_string()),
                is_confidential_amount: Some(a.0.is_confidential_amount.to_string()),
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

    fn show(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return Err(fmt::Error);
        }

        for index in 0..self.contents.len() {
            let none = "[(none)]".to_string();
            let unlimited = "[(unlimited)]".to_string();
            let name = &self.contents[index].name.as_ref().unwrap_or(&none);
            let address = self.fetcher(&self.contents[index].address)?;
            let asset_type = self.fetcher(&self.contents[index].asset_type)?;
            let memo = &self.contents[index].memo.as_ref().unwrap_or(&none);
            let maximun = &self.contents[index].maximun.as_ref().unwrap_or(&unlimited);
            let amount = self.fetcher(&self.contents[index].amount)?;
            let is_transferable = &self.contents[index]
                .is_transferable
                .as_ref()
                .unwrap_or(&none);
            let is_issued = &self.contents[index].is_issued.as_ref().unwrap_or(&none);
            let is_confidential_amount = &self.contents[index]
                .is_confidential_amount
                .as_ref()
                .unwrap_or(&none);

            write!(
                f,
                "
{}
Name:                   {}
ETH Compatible Address: {}
Asset Type:             {}
Memo:                   {}
Maximun:                {}
Amount:                 {}
Is Transferable:        {}
Is Issued:              {}
Is Confidential Amount: {}
            ",
                Emoji("🪙", "$ "),
                style(name).bold().cyan(),
                style(address).bold().cyan(),
                style(asset_type).bold().yellow(),
                style(memo).bold().cyan(),
                style(maximun).bold().cyan(),
                style(amount).bold().cyan(),
                style(is_transferable).bold().magenta(),
                style(is_issued).bold().magenta(),
                style(is_confidential_amount).bold().magenta(),
            )?;
        }
        Ok(())
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
        write!(
            f,
            "
{} {}
{} ETH Compatible Address: {}
{} Asset Type:             {}
",
            Emoji("✨", ":)"),
            style("Success Issued").bold().green(),
            Emoji("★ ", "* "),
            style(address).white(),
            Emoji("★ ", "* "),
            style(asset_type).white(),
        )
    }
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.contents.is_empty() {
            return self.empty(f);
        }

        match self.typ {
            DisplayType::Show => self.show(f),
            DisplayType::Create => self.create(f),
            DisplayType::Issue => self.issue(f),
        }
    }
}
