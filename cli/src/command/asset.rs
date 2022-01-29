use std::{fmt::Display, path::Path};

use crate::entry::{asset as entry_asset, wallet as entry_wallet};

use abcf_sdk::providers::HttpGetProvider;
use anyhow::Result;
use async_compat::Compat;
use clap::{ArgGroup, Parser};
use futures::executor::block_on;
use libfindora::asset::AssetType;
use libfn::{
    entity::{Define as EntityDefine, Entity, Issue as EntityIssue},
    net::send_tx,
    types::SecretKey,
    Builder,
};
use primitive_types::U256;
use rand_chacha::{rand_core::RngCore, rand_core::SeedableRng, ChaChaRng};

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
#[clap(group(ArgGroup::new("from").required(true).args(&["from-address", "from-secret"])))]
struct Create {
    /// To specific an address as the Findora wallet which is
    /// 1. ETH compatible address (0x...)
    /// 2. Findora addreess (fra...)
    #[clap(short = 'f', long, value_name = "ADDRESS", forbid_empty_values = true)]
    from_address: Option<String>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted secret
    #[clap(short = 's', long, value_name = "SECRET", forbid_empty_values = true)]
    from_secret: Option<String>,
    /// Memo is a note for this new asset
    #[clap(long)]
    memo: Option<String>,
    /// Is transferable for the new asset
    #[clap(short = 't', long)]
    is_transferable: bool,
    /// Decimal places to mark the maximum in floating of the new asset
    #[clap(short, long, default_value = "6")]
    decimal_places: u8,
    /// Maximum amount of the new asset
    #[clap(short, long)]
    maximum: Option<u64>,
}

#[derive(Parser, Debug)]
struct Show {
    /// Wallet address to show the asset information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    address: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(group(ArgGroup::new("from").required(true).args(&["from-address", "from-secret"])))]
struct Issue {
    /// To specific an address as the Findora wallet which is
    /// 1. ETH compatible address (0x...)
    /// 2. Findora addreess (fra...)
    #[clap(short = 'f', long, value_name = "ADDRESS", forbid_empty_values = true)]
    from_address: Option<String>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted secret
    #[clap(short = 's', long, value_name = "SECRET", forbid_empty_values = true)]
    from_secret: Option<String>,
    /// Custom code of the new asset
    #[clap(short, long)]
    code: Option<String>,
    /// Amount when issuing an asset
    #[clap(short, long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Is the amount confidential when issuing an asset
    #[clap(short, long)]
    is_confidential_amount: bool,
}

impl Command {
    pub fn execute(&self, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
        match &self.subcmd {
            SubCommand::Create(cmd) => create(cmd, home, addr),
            SubCommand::Show(cmd) => show(cmd),
            SubCommand::Issue(cmd) => issue(cmd, home, addr),
        }
    }
}

fn create(cmd: &Create, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let mut assets = entry_asset::Assets::new(home)?;
    let wallets = entry_wallet::Wallets::new(home)?;
    let wallet = get_wallet(&wallets, &cmd.from_address, &cmd.from_secret)?;
    let maximum = match &cmd.maximum {
        Some(max) => Some(U256::from_str_radix(max.to_string().as_str(), 10)?),
        None => None,
    };

    let mut asset_type: [u8; 32] = [0; 32];
    let mut rng = ChaChaRng::from_entropy();
    rng.try_fill_bytes(&mut asset_type)?;

    let define = Entity::Define(EntityDefine {
        maximum,
        transferable: cmd.is_transferable,
        keypair: SecretKey::from_base64(&wallet.secret)?.key.into_keypair(),
        asset: AssetType(asset_type),
    });

    let mut provider = HttpGetProvider::new(addr);
    let mut builder = Builder::default();
    block_on(Compat::new(builder.from_entities(
        &mut rng,
        &mut provider,
        vec![define],
    )))?;
    block_on(Compat::new(send_tx(
        &mut provider,
        builder.build(&mut rng)?,
    )))?;

    assets.create(&entry_asset::Asset {
        address: wallet.address,
        asset_type: AssetType(asset_type),
        memo: cmd.memo.clone(),
        code: None,
        decimal_place: cmd.decimal_places,
        maximun: cmd.maximum,
        is_transferable: cmd.is_transferable,
        is_issued: false,
    })?;

    Ok(Box::new(0))
}

fn show(_cmd: &Show) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}

fn issue(cmd: &Issue, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let mut assets = entry_asset::Assets::new(home)?;
    let wallets = entry_wallet::Wallets::new(home)?;
    let wallet = get_wallet(&wallets, &cmd.from_address, &cmd.from_secret)?;
    let mut asset = assets.read(&wallet.address)?;

    let issue = Entity::Issue(EntityIssue {
        amount: cmd.amount,
        asset_type: asset.asset_type,
        confidential_amount: cmd.is_confidential_amount,
        keypair: SecretKey::from_base64(&wallet.secret)?.key.into_keypair(),
    });

    let mut provider = HttpGetProvider::new(addr);
    let mut rng = ChaChaRng::from_entropy();
    let mut builder = Builder::default();
    block_on(Compat::new(builder.from_entities(
        &mut rng,
        &mut provider,
        vec![issue],
    )))?;
    block_on(Compat::new(send_tx(
        &mut provider,
        builder.build(&mut rng)?,
    )))?;

    asset.code = cmd.code.clone();
    assets.update(&asset)?;

    Ok(Box::new(0))
}

fn get_wallet(
    wallets: &entry_wallet::Wallets,
    addr: &Option<String>,
    secret: &Option<String>,
) -> Result<entry_wallet::Wallet> {
    let wallet = if let Some(addr) = addr {
        wallets.read().by_address(addr).build()?
    } else if let Some(secret) = secret {
        wallets.read().by_secret(secret).build()?
    } else {
        unreachable!()
    };
    Ok(wallet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_command_asset_execute_create() {
        let home = TempDir::new("test_command_asset_execute_create").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Create(Create {
                from_address: Some("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string()),
                from_secret: None,
                memo: None,
                is_transferable: false,
                decimal_places: 6,
                maximum: None,
            }),
        };

        // because we did not setup the findorad server
        // should be connection refused error
        assert!(cmd.execute(home.path(), "127.0.0.1").is_err());
    }

    #[test]
    fn test_command_asset_execute_issue() {
        let home = TempDir::new("test_command_asset_execute_issue").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Issue(Issue {
                from_address: Some("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string()),
                from_secret: None,
                is_confidential_amount: false,
                code: None,
                amount: 999,
            }),
        };

        // because we did not setup the findorad server
        // should be connection refused error
        assert!(cmd.execute(home.path(), "127.0.0.1").is_err());
    }
}
