use std::path::PathBuf;

use clap::{Parser, ValueHint};

use crate::config::Config;

mod asset;
mod delegate;
mod execute;
mod setup;
mod show;
mod transfer;
mod tx;
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
    pub async fn execute(&self) -> ruc::Result<()> {
        let config = Config::load(&self.home, &self.config)?;

        match &self.subcmd {
            SubCommand::Asset(c) => c.execute(config).await?,
            SubCommand::Delegate(c) => c.execute(config).await?,
            SubCommand::Show(c) => c.execute(config).await?,
            SubCommand::Setup(c) => c.execute(config).await?,
            SubCommand::Execute(c) => c.execute(config).await?,
            SubCommand::Transfer(c) => c.execute(config).await?,
            SubCommand::Wallet(c) => c.execute(config).await?,
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
    /// View validator status and accumulated rewards
    Show(show::Command),
    /// Setup configuration entry
    Setup(setup::Command),
    /// Execute batch of transaction
    Execute(execute::Command),
    /// Transfer asset to other user
    Transfer(transfer::Command),
    /// Manage wallet
    Wallet(wallet::Command),
}
