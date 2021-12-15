use std::path::PathBuf;

use clap::{Parser, ValueHint};

use crate::config::Config;

mod account;
mod asset;
mod claim;
mod contract;
mod delegate;
mod execute;
mod genkey;
mod issue;
mod setup;
mod show;
mod stake;
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
            SubCommand::Account(c) => c.execute(config).await?,
            SubCommand::Asset(c) => c.execute(config).await?,
            SubCommand::Claim(c) => c.execute(config).await?,
            SubCommand::Contract(c) => c.execute(config).await?,
            SubCommand::Delegate(c) => c.execute(config).await?,
            SubCommand::Genkey(c) => c.execute(config).await?,
            SubCommand::Show(c) => c.execute(config).await?,
            SubCommand::Stake(c) => c.execute(config).await?,
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
    /// Return user contract account information
    Account(account::Command),
    /// Manipulate custom asset
    Asset(asset::Command),
    /// Claim accumulated FRA rewards
    Claim(claim::Command),
    /// Manipulate contract
    Contract(contract::Command),
    /// Delegating operations
    Delegate(delegate::Command),
    /// Generating key pair and Ethereum address operations
    Genkey(genkey::Command),
    /// View validator status and accumulated rewards
    Show(show::Command),
    /// Staking operations
    Stake(stake::Command),
    /// Setup configuration entry
    Setup(setup::Command),
    /// Execute batch of transaction
    Execute(execute::Command),
    /// Transfer asset to other user
    Transfer(transfer::Command),
    /// Issue asset
    Issue(issue::Command),
    /// Manage wallet
    Wallet(wallet::Command),
}
