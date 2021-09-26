use std::path::PathBuf;

use clap::{Clap, ValueHint};
use ruc::*;

use crate::config::Config;

mod execute;
mod issue;
mod setup;
mod transfer;
mod wallet;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(short, long, env = "FN_HOME", default_value = concat!(env!("HOME"), "/.findora/fn"), value_hint = ValueHint::DirPath)]
    pub home: PathBuf,
    #[clap(short, long, env = "FN_CONFIG", default_value = concat!(env!("HOME"), "/.findora/fn/config.toml"), value_hint = ValueHint::FilePath)]
    pub config: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn execute(&self) -> Result<()> {
        let config = Config::load(&self.home, &self.config)?;

        match &self.subcmd {
            SubCommand::Setup(c) => c.execute(config)?,
            SubCommand::Execute(c) => c.execute(config)?,
            SubCommand::Transfer(c) => c.execute(config)?,
            SubCommand::Issue(c) => c.execute(config)?,
            SubCommand::Wallet(c) => c.execute(config)?,
        }

        Ok(())
    }
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(version, author, about = "Setup configuration entry.")]
    Setup(setup::Command),
    #[clap(version, author, about)]
    Execute(execute::Command),
    #[clap(version, author, about)]
    Transfer(transfer::Command),
    #[clap(version, author, about)]
    Issue(issue::Command),
    #[clap(version, author, about = "Manage wallet")]
    Wallet(wallet::Command),
}
