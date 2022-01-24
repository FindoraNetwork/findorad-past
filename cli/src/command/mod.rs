use std::{fmt::Display, path::PathBuf};

use crate::config::Config;

use anyhow::Result;
use clap::{Parser, ValueHint};
use lazy_static::lazy_static;

pub(crate) mod asset;
pub(crate) mod delegate;
pub(crate) mod dev;
pub(crate) mod setup;
pub(crate) mod transfer;
pub(crate) mod wallet;

const DEFAULT_FINDORA_FOLDER_NAME: &str = ".findora";
const DEFAULT_FN_FOLDER_NAME: &str = "fn";

lazy_static! {
    static ref DEFAULT_HOME_PATH: String = {
        // must get home!
        home::home_dir()
            .unwrap()
            .join(DEFAULT_FINDORA_FOLDER_NAME)
            .join(DEFAULT_FN_FOLDER_NAME)
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
        let mut config = Config::load(&self.home)?;

        match &self.subcmd {
            SubCommand::Asset(c) => c.execute(config),
            SubCommand::Delegate(c) => c.execute(config),
            SubCommand::Setup(c) => c.execute(&mut config),
            SubCommand::Transfer(c) => c.execute(&self.home),
            SubCommand::Wallet(c) => c.execute(&self.home),
            SubCommand::Dev(c) => c.execute(),
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
    /// Development environment
    Dev(dev::Command),
}
