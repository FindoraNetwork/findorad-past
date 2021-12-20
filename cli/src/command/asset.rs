use std::fmt::Display;

use crate::config::Config;

use anyhow::Result;
use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Create a new asset
    Create(Create),
    /// Show a list of asset addresses or specific one for detail information
    Show(Show),
    /// Issue an asset on ledger
    Issue(Issue),
}

#[derive(Parser, Debug)]
struct Create {
    /// To specific a file path to the Findora wallet which contains base64-formatted XfrPrivateKey
    #[clap(short, long, value_name = "PATH", forbid_empty_values = true, value_hint = ValueHint::FilePath)]
    secret_key: Option<std::path::PathBuf>,
    /// Mnemonic phrase of the new asset
    #[clap(forbid_empty_values = true)]
    memo: String,
    /// Is transferable for the new asset
    #[clap(long)]
    is_transferable: bool,
    /// Decimal places of the new asset
    #[clap(short, long, default_value = "6")]
    decimal_places: u8,
    /// Maximum amount of the new asset
    #[clap(short, long)]
    maximum: Option<u64>,
    /// Custom code of the new asset
    #[clap(short, long)]
    code: Option<String>,
}

#[derive(Parser, Debug)]
struct Show {
    /// Wallet address to show the asset information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    address: Option<String>,
}

#[derive(Parser, Debug)]
struct Issue {
    /// To specific a file path to the Findora wallet which contains base64-formatted XfrPrivateKey
    #[clap(short, long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    secret_key: Option<std::path::PathBuf>,
    /// Custom code of the new asset
    #[clap(short, long)]
    code: Option<String>,
    /// Amount when issuing an asset
    #[clap(short, long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Is hidden the amount when issuing an asset
    #[clap(long)]
    is_hidden: bool,
}

impl Command {
    pub fn execute(&self, _config: Config) -> Result<Box<dyn Display>> {
        match &self.subcmd {
            SubCommand::Create(_create) => {}
            SubCommand::Show(_show) => {}
            SubCommand::Issue(_issue) => {}
        }
        Ok(Box::new(0))
    }
}
