use std::fmt::Display;

use crate::config::Config;
use crate::display::wallet as display_wallet;
use crate::entry::wallet as entry_wallet;

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use libfn::types::Wallet as lib_wallet;

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
    /// Name of the new wallet to be created
    #[clap(short, long, forbid_empty_values = true)]
    name: Option<String>,
}

#[derive(Parser, Debug)]
struct Delete {
    /// Wallet address to do the deletion
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub fn execute(&self, cfg: &Config) -> Result<Box<dyn Display>> {
        let mut wallets = entry_wallet::Wallets::new(&cfg.node.home)
            .with_context(|| format!("wallets new failed: {:?}", cfg.node.home))?;

        match &self.subcmd {
            SubCommand::Show(cmd) => show(cmd, &wallets),
            SubCommand::Create(cmd) => create(cmd, &mut wallets),
            SubCommand::Delete(cmd) => delete(cmd, &mut wallets),
        }
    }
}

fn show(cmd: &Show, wallets: &entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let result = match &cmd.address {
        Some(a) => display_wallet::Display::from(
            wallets
                .read(a)
                .with_context(|| format!("read wallet failed: {:?}", cmd))?,
        ),
        None => display_wallet::Display::from(wallets.list()),
    };

    Ok(Box::new(result))
}

fn delete(cmd: &Delete, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    wallets
        .delete(&cmd.address)
        .with_context(|| format!("delete wallet failed: {:?}", cmd))?;

    Ok(Box::new(display_wallet::Display::from(cmd.address.clone())))
}

fn create(cmd: &Create, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let result = match &cmd.mnemonic {
        Some(m) => lib_wallet::from_mnemonic(&m),
        None => lib_wallet::generate(),
    };

    let wallet = match result {
        Ok(v) => v,
        Err(e) => bail!("lib_wallet creating failed: {:?}", e),
    };

    let result = wallets
        .create(&entry_wallet::Wallet {
            name: cmd.name.clone(),
            mnemonic: wallet.mnemonic,
            address: wallet
                .address
                .to_base64()
                .map_err(|e| anyhow!("lib_wallet address to_base64 failed: {}", e))?,
            public: wallet
                .public
                .to_base64()
                .map_err(|e| anyhow!("lib_wallet public to_base64 failed: {}", e))?,
            secret: wallet
                .secret
                .to_base64()
                .map_err(|e| anyhow!("lib_wallet secret to_base64 failed: {}", e))?,
        })
        .with_context(|| format!("create wallet failed: {:?}", cmd))?;

    Ok(Box::new(display_wallet::Display::from(result.address)))
}
