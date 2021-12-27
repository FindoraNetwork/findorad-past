use std::{fmt::Display, path::Path};

use crate::display::wallet as display_wallet;
use crate::entry::wallet as entry_wallet;

use anyhow::Result;
use clap::Parser;
use libfn::types::Wallet;

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
    /// The
    /// 1. ETH compatible address (0x...) or
    /// 2. Findora addreess (fra...)
    /// to show the wallet detail information of the specific one
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
    /// The
    /// 1. ETH compatible address (0x...) or
    /// 2. Findora addreess (fra...)
    /// to do the deletion
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub fn execute(&self, home: &Path) -> Result<Box<dyn Display>> {
        let mut wallets = entry_wallet::Wallets::new(home)?;

        match &self.subcmd {
            SubCommand::Show(cmd) => show(cmd, &wallets),
            SubCommand::Create(cmd) => create(cmd, &mut wallets),
            SubCommand::Delete(cmd) => delete(cmd, &mut wallets),
        }
    }
}

fn show(cmd: &Show, wallets: &entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let result = match &cmd.address {
        Some(addr) => {
            let wallet = wallets.read().from_address(addr).build()?;

            let c = display_wallet::Content {
                name: wallet.name.clone(),
                eth_compatible_address: Some(wallet.to_eth_address()?),
                fra_address: Some(wallet.to_fra_address()?),
                public_key: Some(wallet.public),
                secret: Some(wallet.secret),
                mnemonic: Some(wallet.mnemonic),
            };
            display_wallet::Display::from(c)
        }
        None => display_wallet::Display::from(wallets.list()?),
    };

    Ok(Box::new(result))
}

fn delete(cmd: &Delete, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    wallets.delete(&cmd.address)?;

    Ok(Box::new(display_wallet::Display::from((
        cmd.address.clone(),
        display_wallet::DisplayType::Delete,
    ))))
}

fn create(cmd: &Create, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let wallet = match &cmd.mnemonic {
        Some(m) => Wallet::from_mnemonic(m)?,
        None => Wallet::generate()?,
    };

    let w = &entry_wallet::Wallet {
        name: cmd.name.clone(),
        mnemonic: wallet.mnemonic,
        address: wallet.address.to_base64()?,
        public: wallet.public.to_base64()?,
        secret: wallet.secret.to_base64()?,
    };
    wallets.create(w)?;

    Ok(Box::new(display_wallet::Display::from((
        entry_wallet::WalletInfo {
            name: w.name.clone(),
            eth_compatible_address: w.to_eth_address()?,
            fra_address: w.to_fra_address()?,
        },
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
        // because the input address is EthereumAddressFormatError
        assert!(cmd.execute(node_home.path()).is_err());
    }
}
