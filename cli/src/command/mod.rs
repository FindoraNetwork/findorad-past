use std::path::PathBuf;

use clap::{Parser, ValueHint};
// use ruc::*;

use crate::config::Config;

mod execute;
mod issue;
mod setup;
mod transfer;
mod tx;
mod wallet;

#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(short, long, default_value = concat!(env!("HOME"), "/.findora/fn"), value_hint = ValueHint::DirPath)]
    pub home: PathBuf,
    #[clap(short, long, default_value = concat!(env!("HOME"), "/.findora/fn/config.toml"), value_hint = ValueHint::FilePath)]
    pub config: PathBuf,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub async fn execute(&self) -> ruc::Result<()> {
        let config = Config::load(&self.home, &self.config)?;

        match &self.subcmd {
            SubCommand::Setup(c) => c.execute(config).await?,
            SubCommand::Execute(c) => c.execute(config).await?,
            SubCommand::Transfer(c) => c.execute(config).await?,
            SubCommand::Issue(c) => c.execute(config).await?,
            SubCommand::Wallet(c) => c.execute(config).await?,
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(version, author, about = "Setup configuration entry.")]
    Setup(setup::Command),
    #[clap(version, author, about = "Execute batch of transaction.")]
    Execute(execute::Command),
    #[clap(version, author, about = "Transfer asset to other user.")]
    Transfer(transfer::Command),
    #[clap(version, author, about = "Issue asset.")]
    Issue(issue::Command),
    #[clap(version, author, about = "Manage wallet")]
    Wallet(wallet::Command),
}
