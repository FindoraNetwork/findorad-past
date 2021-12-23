use std::{fmt::Display, path::Path};

use crate::display::wallet as display_wallet;
use crate::entry::wallet as entry_wallet;

use anyhow::Result;
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
    /// Use a specific wallet as current in use wallet
    Use(Use),
}

#[derive(Parser, Debug)]
struct Show {
    /// The ETH compatible address to show the wallet information of the specific one
    #[clap(short, long, forbid_empty_values = true, group = "input")]
    address: Option<String>,
    /// Showing the current in use wallet
    #[clap(short, long, group = "input")]
    current: bool,
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

#[derive(Parser, Debug)]
struct Use {
    /// The ETH compatible address to do the deletion
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
            SubCommand::Use(cmd) => use_this(cmd, &mut wallets),
        }
    }
}

fn use_this(cmd: &Use, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    wallets.set_current(&types::Address::from_eth(&cmd.address)?.to_base64()?)?;

    Ok(Box::new(display_wallet::Display::from((
        cmd.address.clone(),
        display_wallet::DisplayType::Use,
    ))))
}

fn show(cmd: &Show, wallets: &entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    fn convert_helper(wallet: &entry_wallet::Wallet) -> Result<display_wallet::Content> {
        Ok(display_wallet::Content {
            name: wallet.name.clone(),
            eth_compatible_address: Some(types::Address::from_base64(&wallet.address)?.to_eth()?),
            fra_address: Some(types::PublicKey::from_base64(&wallet.public)?.to_bech32()?),
            public_key: Some(wallet.public.clone()),
            secret: Some(wallet.secret.clone()),
            mnemonic: Some(wallet.mnemonic.clone()),
            in_use: Some(if wallet.current {
                "yes".to_string()
            } else {
                "no".to_string()
            }),
        })
    }

    let result = match &cmd.address {
        Some(a) => {
            let wallet = wallets.read(&types::Address::from_eth(a)?.to_base64()?)?;
            display_wallet::Display::from(convert_helper(&wallet)?)
        }
        None => {
            if cmd.current {
                let wallet = wallets.get_current()?;
                display_wallet::Display::from(convert_helper(&wallet)?)
            } else {
                let mut list = vec![];
                for w in wallets.list().iter() {
                    list.push(entry_wallet::ListWallet {
                        name: w.name.clone(),
                        address: types::Address::from_base64(&w.address)?.to_eth()?,
                        current: w.current,
                    });
                }
                display_wallet::Display::from(list)
            }
        }
    };

    Ok(Box::new(result))
}

fn delete(cmd: &Delete, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    wallets.delete(&types::Address::from_eth(&cmd.address)?.to_base64()?)?;

    Ok(Box::new(display_wallet::Display::from((
        cmd.address.clone(),
        display_wallet::DisplayType::Delete,
    ))))
}

fn create(cmd: &Create, wallets: &mut entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let wallet = match &cmd.mnemonic {
        Some(m) => types::Wallet::from_mnemonic(m)?,
        None => types::Wallet::generate()?,
    };

    wallets.create(&entry_wallet::Wallet {
        name: cmd.name.clone(),
        mnemonic: wallet.mnemonic,
        address: wallet.address.to_base64()?,
        public: wallet.public.to_base64()?,
        secret: wallet.secret.to_base64()?,
        current: false,
    })?;

    Ok(Box::new(display_wallet::Display::from((
        wallet.address.to_eth()?,
        display_wallet::DisplayType::Create,
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_command_wallet_execute_use() {
        let node_home = TempDir::new("test_command_wallet_execute_use").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Use(Use {
                address: "some_address".to_string(),
            }),
        };
        // because not found
        assert!(cmd.execute(node_home.path()).is_err());
    }

    #[test]
    fn test_command_wallet_execute_show() {
        let node_home = TempDir::new("test_command_wallet_execute_show").unwrap();
        let mut cmd = Command {
            subcmd: SubCommand::Show(Show {
                address: None,
                current: false,
            }),
        };
        assert!(cmd.execute(node_home.path()).is_ok());

        cmd.subcmd = SubCommand::Show(Show {
            address: Some("some_address".to_string()),
            current: false,
        });
        // because not found
        assert!(cmd.execute(node_home.path()).is_err());

        cmd.subcmd = SubCommand::Show(Show {
            address: None,
            current: true,
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
