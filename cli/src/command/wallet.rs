use crate::config::Config;

use anyhow::Result;
use clap::{ArgEnum, Parser};

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Show a list of wallet addresses or specific one for detail information
    Show(Show),
    /// Create a new wallet or create a new wallet from Mnemonic phrase
    Create(Create),
    /// Delete a wallet
    Delete(Delete),
}

#[derive(Parser, Debug)]
struct Show {
    /// Wallet address to show the wallet information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    address: Option<String>,
}

#[derive(Parser, Debug)]
struct Create {
    /// Specific a wallet type to create
    #[clap(arg_enum, short, long, default_value = "findora")]
    wallet_typ: KeyType,
    /// Specific to create a new wallet from Mnemonic phrase
    #[clap(short, long, forbid_empty_values = true)]
    mnemonic: Option<String>,
}

#[derive(ArgEnum, Debug, Clone)]
enum KeyType {
    /// Generate a random Findora public and private key pair
    FRA,
    /// Generate an Ethereum public and private key pair from??? and memo too???
    ETH,
}

#[derive(Parser, Debug)]
struct Delete {
    /// Wallet address to do the deletion
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub fn execute(&self, _config: Config) -> Result<()> {
        match &self.subcmd {
            SubCommand::Show(_show) => {}
            SubCommand::Create(_create) => {}
            SubCommand::Delete(_delete) => {}
        }
        Ok(())
    }
}
