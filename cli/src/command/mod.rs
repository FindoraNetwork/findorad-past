use std::path::PathBuf;

use crate::config::Config;

use anyhow::Result;
use clap::{Parser, ValueHint};

mod asset;
mod delegate;
mod setup;
mod transfer;
mod wallet;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(long, default_value = concat!(env!("HOME"), "/.findora/fn"),value_name = "FOLDER", value_hint = ValueHint::DirPath)]
    pub home: PathBuf,
    #[clap(long, default_value = concat!(env!("HOME"), "/.findora/fn/config.toml"), value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub config: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn execute(&self) -> Result<()> {
        let config = Config::new(&self.home, &self.config)?;

        match &self.subcmd {
            SubCommand::Asset(c) => c.execute(config)?,
            SubCommand::Delegate(c) => c.execute(config)?,
            SubCommand::Setup(c) => c.execute(config)?,
            SubCommand::Transfer(c) => c.execute(config)?,
            SubCommand::Wallet(c) => c.execute(config)?,
        }

        Ok(())
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
