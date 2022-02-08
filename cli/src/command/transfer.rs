// use std::future::poll_fn;
use std::{fmt::Display, path::Path};

use crate::{
    display::transfer as display_transfer,
    entry::{transfer as entry_transfer, wallet as entry_wallet},
};

use anyhow::Result;
use async_compat::Compat;
use clap::{ArgGroup, Parser};
use libfn::{
    entity,
    types::{Address, SecretKey},
    Builder,
};

use abcf_sdk::providers::HttpGetProvider;
use futures::executor::block_on;
use rand_chacha::rand_core::SeedableRng;
use rand_chacha::ChaChaRng;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Send tokens from the wallet to a specific address directly
    Send(Send),
    /// Save this sending request for further bach sending them together
    Save(Save),
    /// Batch sending the saved requests
    Batch(Batch),
    /// Show a list of saved batch process names or a specific one for detailed information
    Show(Show),
}

#[derive(Parser, Debug)]
#[clap(group(ArgGroup::new("from").required(true).args(&["from-address", "from-secret"])))]
struct Send {
    /// To specific an address as the Findora wallet which is
    /// 1. ETH compatible address (0x...)
    /// 2. Findora addreess (fra...)
    #[clap(short = 'f', long, value_name = "ADDRESS", forbid_empty_values = true)]
    from_address: Option<String>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted secret
    #[clap(short = 's', long, value_name = "SECRET", forbid_empty_values = true)]
    from_secret: Option<String>,
    /// Asset Type to send which is a base64-formatted string
    #[clap(short = 'a', long, required = true, forbid_empty_values = true)]
    asset_type: String,
    /// Amount of the Asset Type to send
    #[clap(short = 'm', long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address to send which is a
    /// 1. ETH compatible address (0x...) or
    /// 2. Findora addreess (fra...)
    #[clap(short = 't', long, required = true, forbid_empty_values = true)]
    to_address: String,
    /// Make the amount confidential in the transaction
    #[clap(short = 'M', long)]
    is_confidential_amount: bool,
    /// Make the asset code confidential in the transaction
    #[clap(short = 'A', long)]
    is_confidential_asset: bool,
}

#[derive(Parser, Debug)]
struct Save {
    /// Name of the batch process for identifying in the batch command
    /// Save with the same batch name will appending the new request
    #[clap(short = 'n', long, required = true, forbid_empty_values = true)]
    batch_name: String,
    #[clap(flatten)]
    request: Send,
}

#[derive(Parser, Debug)]
struct Batch {
    /// Name of the batch process will be executing
    #[clap(forbid_empty_values = true)]
    batch_name: String,
}

#[derive(Parser, Debug)]
struct Show {
    /// Name of the batch process to show the request information of the specific one
    #[clap(short = 'n', long, forbid_empty_values = true)]
    batch_name: Option<String>,
}

impl Command {
    pub fn execute(&self, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
        match &self.subcmd {
            SubCommand::Send(cmd) => send(cmd, home, addr),
            SubCommand::Save(cmd) => save(cmd, home),
            SubCommand::Batch(cmd) => batch(cmd, home, addr),
            SubCommand::Show(cmd) => show(cmd, home),
        }
    }
}

fn send(cmd: &Send, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let secret = get_secret(home, &cmd.from_address, &cmd.from_secret)?;

    send_tx(
        addr,
        vec![entity::Entity::Transfer(
            entity::Transfer::builder()
                .from(&secret.to_base64()?)
                .public_key(&secret.to_public().to_base64()?)
                .address(&entry_wallet::detect_address(&cmd.to_address)?)
                .amount(cmd.amount)
                .asset_type(&cmd.asset_type)
                .confidential_amount(cmd.is_confidential_amount)
                .confidential_asset(cmd.is_confidential_asset)
                .build()?,
        )],
    )?;

    Ok(Box::new(display_transfer::Display::from((
        secret.to_public().to_address()?.to_eth()?,
        cmd.to_address.clone(),
        cmd.amount,
    ))))
}

fn save(cmd: &Save, home: &Path) -> Result<Box<dyn Display>> {
    let secret = get_secret(home, &cmd.request.from_address, &cmd.request.from_secret)?;

    entry_transfer::Transfers::new(home)?.create(&entry_transfer::Transfer {
        name: cmd.batch_name.clone(),
        from_secret: secret.to_base64()?,
        to_base64_address: entry_wallet::detect_address(&cmd.request.to_address)?,
        public_key: secret.to_public().to_base64()?,
        amount: cmd.request.amount,
        asset_type: cmd.request.asset_type.clone(),
        is_confidential_amount: cmd.request.is_confidential_amount,
        is_confidential_asset: cmd.request.is_confidential_asset,
    })?;

    Ok(Box::new(display_transfer::Display::from((
        cmd.batch_name.clone(),
        display_transfer::DisplayType::Save,
    ))))
}

fn batch(cmd: &Batch, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let mut tfs = entry_transfer::Transfers::new(home)?;
    let transfers = tfs.read(&cmd.batch_name)?;
    let mut entities = Vec::with_capacity(transfers.len());

    for t in transfers {
        entities.push(entity::Entity::Transfer(
            entity::Transfer::builder()
                .from(&t.from_secret)
                .public_key(&t.public_key)
                .address(&t.to_base64_address)
                .amount(t.amount)
                .asset_type(&t.asset_type)
                .confidential_amount(t.is_confidential_amount)
                .confidential_asset(t.is_confidential_asset)
                .build()?,
        ));
    }

    send_tx(addr, entities)?;
    tfs.delete(&cmd.batch_name)?;

    Ok(Box::new(display_transfer::Display::from((
        cmd.batch_name.clone(),
        display_transfer::DisplayType::Batch,
    ))))
}

fn show(cmd: &Show, home: &Path) -> Result<Box<dyn Display>> {
    let transfers = entry_transfer::Transfers::new(home)?;
    match &cmd.batch_name {
        Some(name) => {
            let tfs = transfers.read(name)?;
            let mut contents = Vec::with_capacity(tfs.len());

            for t in tfs {
                contents.push(display_transfer::Content {
                    name: Some(name.to_string()),
                    from_address: Some(
                        SecretKey::from_base64(&t.from_secret)?
                            .to_public()
                            .to_address()?
                            .to_eth()?,
                    ),
                    to_address: Some(Address::from_base64(&t.to_base64_address)?.to_eth()?),
                    public_key: Some(t.public_key.clone()),
                    amount: Some(t.amount.to_string()),
                    asset_type: Some(t.asset_type.clone()),
                    is_confidential_amount: Some(t.is_confidential_amount.to_string()),
                    is_confidential_asset: Some(t.is_confidential_asset.to_string()),
                    ..Default::default()
                });
            }

            Ok(Box::new(display_transfer::Display::from(contents)))
        }
        None => Ok(Box::new(display_transfer::Display::from(
            transfers
                .list()
                .into_iter()
                .map(|name| {
                    (
                        name.clone(),
                        transfers.read(&name).unwrap_or_default().len(),
                    )
                })
                .collect::<Vec<(String, usize)>>(),
        ))),
    }
}

fn send_tx(addr: &str, entities: Vec<entity::Entity>) -> Result<()> {
    let mut prng = ChaChaRng::from_entropy();
    let mut provider = HttpGetProvider::new(addr);
    let mut builder = Builder::default();

    block_on(Compat::new(builder.from_entities(
        &mut prng,
        &mut provider,
        entities,
    )))?;

    let tx = builder.build(&mut prng)?;
    tx.serialize()?;
    Ok(())
}

fn get_secret(home: &Path, addr: &Option<String>, secret: &Option<String>) -> Result<SecretKey> {
    if let Some(addr) = addr {
        Ok(SecretKey::from_base64(
            &entry_wallet::Wallets::new(home)?
                .read()
                .by_address(addr)
                .build()?
                .secret,
        )?)
    } else if let Some(secret) = secret {
        Ok(SecretKey::from_base64(secret)?)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_command_transfer_execute_send() {
        let home = TempDir::new("test_command_transfer_execute_send").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Send(Send {
                from_address: Some("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string()),
                from_secret: None,
                asset_type: "1TYZSwkxQI6-q49vgFsCOuXaOjaHbhtEV2GyDoPglUU=".to_string(),
                amount: 99,
                to_address: "0x283590e19dee343ea0a8f4ecec906d53308068b5".to_string(),
                is_confidential_amount: false,
                is_confidential_asset: false,
            }),
        };

        // because we did not setup the findorad server
        // should be connection refused error
        assert!(cmd.execute(home.path(), "127.0.0.1").is_err());
    }

    #[test]
    fn test_command_transfer_execute_save_show_batch() {
        let home = TempDir::new("test_command_transfer_execute_save_show_batch").unwrap();
        entry_wallet::Wallets::new(home.path())
            .unwrap()
            .create(&entry_wallet::Wallet {
                name: None,
                mnemonic: "".to_string(),
                address: "KDWQ4Z3uND6gqPTs7JBtUzCAaLU=".to_string(),
                public: "".to_string(),
                secret: "_12euPXJxDbpcw7fMNJufUZgrTgcK7ShTJmXuZZe8eM".to_string(),
            })
            .unwrap();

        let cmd = Command {
            subcmd: SubCommand::Save(Save {
                batch_name: "test_command_transfer_execute_save_show_batch".to_string(),
                request: Send {
                    from_address: Some("0x283590e19dee343ea0a8f4ecec906d53308068b5".to_string()),
                    from_secret: None,
                    asset_type: "1TYZSwkxQI6-q49vgFsCOuXaOjaHbhtEV2GyDoPglUU=".to_string(),
                    amount: 99,
                    to_address: "0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string(),
                    is_confidential_amount: false,
                    is_confidential_asset: false,
                },
            }),
        };
        assert!(cmd.execute(home.path(), "127.0.0.1").is_ok());

        let cmd = Command {
            subcmd: SubCommand::Show(Show { batch_name: None }),
        };
        assert!(cmd.execute(home.path(), "127.0.0.1").is_ok());

        let cmd = Command {
            subcmd: SubCommand::Show(Show {
                batch_name: Some("test_command_transfer_execute_save_show_batch".to_string()),
            }),
        };
        assert!(cmd.execute(home.path(), "127.0.0.1").is_ok());

        let cmd = Command {
            subcmd: SubCommand::Batch(Batch {
                batch_name: "test_command_transfer_execute_save_show_batch".to_string(),
            }),
        };
        // because we did not setup the findorad server
        // should be connection refused error
        assert!(cmd.execute(home.path(), "127.0.0.1").is_err());
    }
}
