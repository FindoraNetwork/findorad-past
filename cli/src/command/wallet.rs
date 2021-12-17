use clap::Parser;
use libfn::types::Wallet;

use crate::{config::Config, entry::wallet};

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
    /// Specific to create a new wallet from Mnemonic phrase
    #[clap(short, long, forbid_empty_values = true)]
    mnemonic: Option<String>,
}

#[derive(Parser, Debug)]
struct Delete {
    /// Wallet address to do the deletion
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        match &self.subcmd {
            SubCommand::Show(_show) => {}
            SubCommand::Create(_create) => {}
            SubCommand::Delete(_delete) => {}
        }
        Ok(())
    }

    async fn create(&self, cfg: Config, opt: Create) -> ruc::Result<()> {
        let w = match opt.mnemonic {
            Some(m) => Wallet::from_mnemonic(&m)?,
            None => Wallet::generate()?,
        };

        Ok(())
    }
}
