use std::{fmt::Display, path::Path};

use crate::display::wallet as display_wallet;
use crate::entry::wallet as entry_wallet;

use anyhow::{bail, Result};
use clap::{ArgGroup, Parser};
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
#[clap(group(ArgGroup::new("from").required(false).args(&["from-address", "from-secret"])))]
struct Show {
    /// To specific an address as the Findora wallet which is
    /// 1. ETH compatible address (0x...)
    /// 2. Findora addreess (fra...)
    /// to show the wallet detail information
    #[clap(short = 'f', long, value_name = "ADDRESS", forbid_empty_values = true)]
    from_address: Option<String>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted secret to show the wallet detail information
    #[clap(short = 's', long, value_name = "SECRET", forbid_empty_values = true)]
    from_secret: Option<String>,
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
#[clap(group(ArgGroup::new("from").required(true).args(&["from-address", "from-secret"])))]
struct Delete {
    /// To specific an address as the Findora wallet which is
    /// 1. ETH compatible address (0x...)
    /// 2. Findora addreess (fra...)
    /// to do the deletion
    #[clap(short = 'f', long, value_name = "ADDRESS", forbid_empty_values = true)]
    from_address: Option<String>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted secret to do the deletion
    #[clap(short = 's', long, value_name = "SECRET", forbid_empty_values = true)]
    from_secret: Option<String>,
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
    let result = match get_wallet(wallets, &cmd.from_address, &cmd.from_secret) {
        Ok(wallet) => {
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
        Err(e) => match e.downcast_ref::<NoSourceError>() {
            Some(_) => display_wallet::Display::from(wallets.list()?),
            None => bail!(e),
        },
    };

    Ok(Box::new(result))
}

fn delete(cmd: &Delete, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let wallet = get_wallet(wallets, &cmd.from_address, &cmd.from_secret)?;
    wallets.delete(&wallet.to_eth_address()?)?;

    Ok(Box::new(display_wallet::Display::from((
        wallet.to_eth_address()?,
        display_wallet::DisplayType::Delete,
    ))))
}

#[derive(Debug)]
struct NoSourceError;
impl std::fmt::Display for NoSourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "No Source Inputed To Find The Wallet") // user-facing output
    }
}

fn get_wallet(
    wallets: &entry_wallet::Wallets,
    addr: &Option<String>,
    secret: &Option<String>,
) -> Result<entry_wallet::Wallet> {
    if let Some(addr) = addr {
        wallets.read().by_address(addr).build()
    } else if let Some(secret) = secret {
        wallets.read().by_secret(secret).build()
    } else {
        bail!(NoSourceError)
    }
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
            subcmd: SubCommand::Show(Show {
                from_address: None,
                from_secret: None,
            }),
        };
        assert!(cmd.execute(node_home.path()).is_ok());
        cmd.subcmd = SubCommand::Show(Show {
            from_address: Some("some_address".to_string()),
            from_secret: None,
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
                from_address: Some("some_address".to_string()),
                from_secret: None,
            }),
        };
        // because the input address is EthereumAddressFormatError
        assert!(cmd.execute(node_home.path()).is_err());
    }
}
