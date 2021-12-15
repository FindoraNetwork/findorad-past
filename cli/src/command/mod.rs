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
    #[clap(version, author, about = "Return user contract account information")]
    Account(account::Command),
    #[clap(version, author, about = "Manipulate custom asset")]
    Asset(asset::Command),
    #[clap(version, author, about = "Claim accumulated FRA rewards")]
    Claim(claim::Command),
    #[clap(version, author, about = "Manipulate contract")]
    Contract(contract::Command),
    #[clap(version, author, about = "Delegating operations")]
    Delegate(delegate::Command),
    #[clap(
        version,
        author,
        about = "Generating key pair and Ethereum address operations"
    )]
    Genkey(genkey::Command),
    #[clap(
        version,
        author,
        about = "View validator status and accumulated rewards"
    )]
    Show(show::Command),
    #[clap(version, author, about = "Staking operations")]
    Stake(stake::Command),
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
