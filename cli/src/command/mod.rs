use std::{fmt::Display, path::PathBuf};

use crate::config::Config;

use anyhow::Result;
use clap::{Parser, ValueHint};
use lazy_static::lazy_static;

pub(crate) mod asset;
pub(crate) mod delegate;
pub(crate) mod setup;
pub(crate) mod transfer;
pub(crate) mod wallet;

lazy_static! {
    static ref DEFAULT_HOME_PATH: String = {
        // must get home!
        home::home_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    };
}

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(long, default_value = &DEFAULT_HOME_PATH, value_name = "FOLDER", value_hint = ValueHint::DirPath)]
    home: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn execute(&self) -> Result<Box<dyn Display>> {
        let config = Config::new(&self.home, &self.config)?;

        match &self.subcmd {
            SubCommand::Asset(c) => c.execute(config),
            SubCommand::Delegate(c) => c.execute(config),
            SubCommand::Setup(c) => c.execute(config),
            SubCommand::Transfer(c) => c.execute(config),
            SubCommand::Wallet(c) => c.execute(&self.home),
        }
    }
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Manipulate custom asset
    Asset(asset::Command),
    /// Delegating operations
    Delegate(delegate::Command),
    /// Setup configuration entry
    Setup(setup::Command),
    /// Transfer asset to other user
    Transfer(transfer::Command),
    /// Manage wallet
    Wallet(wallet::Command),
}
