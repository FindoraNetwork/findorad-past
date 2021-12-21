use std::path::PathBuf;

use crate::config::Config;

use anyhow::Result;
use clap::{Parser, ValueHint};

pub(crate) mod asset;
pub(crate) mod delegate;
pub(crate) mod setup;
pub(crate) mod transfer;
pub(crate) mod wallet;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(long, value_name = "FOLDER", value_hint = ValueHint::DirPath)]
    home: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn execute(&self) -> Result<()> {
        let config = Config::load(&self.home)?;

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
