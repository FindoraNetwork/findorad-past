use std::{fmt::Display, path::Path};

use crate::display::wallet as display_wallet;
use crate::entry::wallet as entry_wallet;

use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use libfn::types;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Show a list of wallet addresses or a specific one for detailed information
    Show(Show),
    /// Create a new wallet or create a new wallet from a Mnemonic phrase
    Create(Create),
    /// Delete a wallet
    Delete(Delete),
}

#[derive(Parser, Debug)]
struct Show {
    /// The ETH compatible address to show the wallet information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    address: Option<String>,
}

#[derive(Parser, Debug)]
struct Create {
    /// Specific to create a new wallet from a Mnemonic phrase
    #[clap(short, long, forbid_empty_values = true)]
    mnemonic: Option<String>,
    /// Name of the new wallet to be created
    #[clap(short, long, forbid_empty_values = true)]
    name: Option<String>,
}

#[derive(Parser, Debug)]
struct Delete {
    /// The ETH compatible address to do the deletion
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub fn execute(&self, node_home: &Path) -> Result<Box<dyn Display>> {
        let mut wallets = entry_wallet::Wallets::new(node_home)
            .with_context(|| format!("wallets new failed: {:?}", node_home))?;

        match &self.subcmd {
            SubCommand::Show(cmd) => show(cmd, &wallets),
            SubCommand::Create(cmd) => create(cmd, &mut wallets),
            SubCommand::Delete(cmd) => delete(cmd, &mut wallets),
        }
    }
}

fn show(cmd: &Show, wallets: &entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let result = match &cmd.address {
        Some(a) => {
            let a = types::Address::from_eth(a)
                .map_err(|e| anyhow!("lib_wallet address from_eth failed: {}", e))?
                .to_base64()
                .map_err(|e| anyhow!("lib_wallet address to_bash64 failed: {}", e))?;
            let wallet = wallets
                .read(&a)
                .with_context(|| format!("read wallet failed: {:?}", cmd))?;
            let c = display_wallet::Content {
                name: wallet.name,
                eth_compatible_address: Some(
                    types::Address::from_base64(&wallet.address)
                        .map_err(|e| anyhow!("lib_wallet address from_base64 failed: {}", e))?
                        .to_eth()
                        .map_err(|e| anyhow!("lib_wallet address to_eth failed: {}", e))?,
                ),
                fra_address: Some(
                    types::PublicKey::from_base64(&wallet.public)
                        .map_err(|e| anyhow!("lib_wallet public from_base64 failed: {}", e))?
                        .to_bech32()
                        .map_err(|e| anyhow!("lib_wallet public to_bech32 failed: {}", e))?,
                ),
                public_key: Some(wallet.public),
                secret: Some(wallet.secret),
                mnemonic: Some(wallet.mnemonic),
            };
            display_wallet::Display::from(c)
        }
        None => {
            let mut list = vec![];
            for w in wallets.list().iter() {
                list.push(entry_wallet::ListWallet {
                    name: w.name.clone(),
                    address: types::Address::from_base64(&w.address)
                        .map_err(|e| anyhow!("lib_wallet address from_base64 failed: {}", e))?
                        .to_eth()
                        .map_err(|e| anyhow!("lib_wallet address to_eth failed: {}", e))?,
                });
            }
            display_wallet::Display::from(list)
        }
    };

    Ok(Box::new(result))
}

fn delete(cmd: &Delete, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    wallets
        .delete(&cmd.address)
        .with_context(|| format!("delete wallet failed: {:?}", cmd))?;

    Ok(Box::new(display_wallet::Display::from((
        cmd.address.clone(),
        display_wallet::DisplayType::Delete,
    ))))
}

fn create(cmd: &Create, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let result = match &cmd.mnemonic {
        Some(m) => types::Wallet::from_mnemonic(&m),
        None => types::Wallet::generate(),
    };

    let wallet = match result {
        Ok(v) => v,
        Err(e) => bail!("lib_wallet creating failed: {:?}", e),
    };

    wallets
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

    Ok(Box::new(display_wallet::Display::from((
        wallet
            .address
            .to_eth()
            .map_err(|e| anyhow!("lib_wallet address to_eth failed: {}", e))?,
        display_wallet::DisplayType::Create,
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_command_wallet_execute_show() {
        let node_home = TempDir::new("test_command_wallet_execute_show").unwrap();
        let mut cmd = Command {
            subcmd: SubCommand::Show(Show { address: None }),
        };
        assert!(cmd.execute(node_home.path()).is_ok());
        cmd.subcmd = SubCommand::Show(Show {
            address: Some("some_address".to_string()),
        });
        // because not found
        assert!(cmd.execute(node_home.path()).is_err());
    }

    #[test]
    fn test_command_wallet_execute_create() {
        let node_home = TempDir::new("test_command_wallet_execute_create").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Create(Create {
                mnemonic: None,
                name: None,
            }),
        };
        assert!(cmd.execute(node_home.path()).is_ok());
    }

    #[test]
    fn test_command_wallet_execute_delete() {
        let node_home = TempDir::new("test_command_wallet_execute_delete").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Delete(Delete {
                address: "some_address".to_string(),
            }),
        };
        assert!(cmd.execute(node_home.path()).is_ok());
    }
}
