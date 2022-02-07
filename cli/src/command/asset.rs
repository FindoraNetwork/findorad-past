use std::{collections::HashMap, fmt::Display, path::Path};

use crate::{
    display::asset as display_asset,
    entry::{asset as entry_asset, wallet as entry_wallet},
};

use abcf::ToBytes;
use abcf_sdk::providers::HttpGetProvider;
use anyhow::Result;
use async_compat::Compat;
use clap::{ArgGroup, Parser};
use futures::executor::block_on;
use libfindora::Address;
use libfn::{
    entity::{Define as EntityDefine, Entity, Issue as EntityIssue},
    net::{owned_outputs, send_tx},
    types::SecretKey,
    utils::open_outputs,
    Builder,
};
use primitive_types::U256;
use rand_chacha::{rand_core::SeedableRng, ChaChaRng};

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Create a new asset
    Create(Create),
    /// Show a list of assets information
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
    #[clap(short = 'e', long)]
    memo: Option<String>,
    /// Is transferable for the new asset
    #[clap(short = 'T', long)]
    is_transferable: bool,
    /// Decimal place to mark the maximum in floating of the new asset
    #[clap(short, long, default_value = "6")]
    decimal_place: u8,
    /// Maximum amount of the new asset
    #[clap(short, long)]
    maximum: Option<u64>,
    /// Custom name of the new asset
    #[clap(short, long)]
    name: Option<String>,
}

#[derive(Parser, Debug)]
#[clap(group(ArgGroup::new("from").required(true).args(&["from-address", "from-secret"])))]
struct Show {
    /// To specific an address as the Findora wallet which is
    /// 1. ETH compatible address (0x...)
    /// 2. Findora addreess (fra...)
    #[clap(short = 'f', long, value_name = "ADDRESS", forbid_empty_values = true)]
    from_address: Option<String>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted secret
    #[clap(short = 's', long, value_name = "SECRET", forbid_empty_values = true)]
    from_secret: Option<String>,
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
    /// To specific a plain-text input as the AssetType which is a base64-formatted string
    #[clap(short = 't', long, forbid_empty_values = true)]
    asset_type: String,
    /// Amount when issuing an asset
    #[clap(short = 'm', long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Is the amount confidential when issuing an asset
    #[clap(short = 'M', long)]
    is_confidential_amount: bool,
}

impl Command {
    pub fn execute(&self, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
        match &self.subcmd {
            SubCommand::Create(cmd) => create(cmd, home, addr),
            SubCommand::Show(cmd) => show(cmd, home, addr),
            SubCommand::Issue(cmd) => issue(cmd, home, addr),
        }
    }
}

fn create(cmd: &Create, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let secret = get_secret(home, &cmd.from_address, &cmd.from_secret)?;
    let maximum = match &cmd.maximum {
        Some(max) => Some(U256::from_str_radix(max.to_string().as_str(), 10)?),
        None => None,
    };

    let mut asset = entry_asset::Asset::new();
    let define = Entity::Define(EntityDefine {
        maximum,
        transferable: cmd.is_transferable,
        keypair: secret.key.clone().into_keypair(),
        asset: asset.asset_type,
    });

    let mut provider = HttpGetProvider::new(addr);
    let mut rng = ChaChaRng::from_entropy();
    let mut builder = Builder::default();
    block_on(Compat::new(builder.from_entities(
        &mut rng,
        &mut provider,
        vec![define],
    )))?;
    block_on(Compat::new(send_tx(
        &mut provider,
        // TODO: change back to ? when abcf error type is satisfying
        builder.build(&mut rng)?.to_bytes().unwrap(),
    )))?;

    let mut assets = entry_asset::Assets::new(home)?;
    asset.address = secret.to_public().to_address()?.to_eth()?;
    asset.name = cmd.name.clone();
    asset.memo = cmd.memo.clone();
    asset.decimal_place = cmd.decimal_place;
    asset.maximum = cmd.maximum;
    asset.is_transferable = cmd.is_transferable;
    assets.create(&asset)?;

    Ok(Box::new(display_asset::Display::new(
        display_asset::DisplayType::Create,
        vec![(asset, None)],
    )))
}

fn show(cmd: &Show, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let secret = get_secret(home, &cmd.from_address, &cmd.from_secret)?;
    let mut provider = HttpGetProvider::new(addr);

    let (_, entrypt_output) = block_on(Compat::new(owned_outputs::get(
        &mut provider,
        &Address::from(secret.to_public().key),
    )))?;

    let output = open_outputs(entrypt_output, &secret.key.clone().into_keypair())?;
    let mut output_map = HashMap::with_capacity(output.len());

    for o in output.iter() {
        *output_map
            .entry(*o.open_asset_record.get_asset_type())
            .or_insert(0) += o.open_asset_record.get_amount();
    }

    // mapping the local non-issued assets to be shown
    // caution: if the blockchain network has been reset (very unlikely in production)
    // then the display will be corrupted
    let assets = entry_asset::Assets::new(home)?;
    let address = secret.to_public().to_address()?.to_eth()?;
    for a in assets.list(&address) {
        output_map.entry(a.asset_type).or_insert(0);
    }

    let mut result = vec![];
    for (asset_type, amount) in output_map {
        let mut asset = entry_asset::Asset::from(asset_type);
        // an asset that can be found in the blockchain will have below three attributes
        asset.address = address.clone();
        asset.is_issued = true;
        // TODO: confirm this one,
        // because while testing if an asset is not transferable will be not shown in the blockchain response
        asset.is_transferable = true;

        if let Ok(a) = assets.read(&address, &asset.get_asset_type_base64()) {
            // remap the asset information if it can be found in the local file
            asset = a;
        }
        result.push((asset, Some(amount)));
    }

    Ok(Box::new(display_asset::Display::new(
        display_asset::DisplayType::Show,
        result,
    )))
}

fn issue(cmd: &Issue, home: &Path, addr: &str) -> Result<Box<dyn Display>> {
    let secret = get_secret(home, &cmd.from_address, &cmd.from_secret)?;
    let mut asset = match entry_asset::Assets::new(home)?
        .read(&secret.to_public().to_address()?.to_eth()?, &cmd.asset_type)
    {
        Ok(a) => a,
        Err(_) => entry_asset::Asset::new_from_asset_type_base64(&cmd.asset_type)?,
    };

    let issue = Entity::Issue(EntityIssue {
        amount: cmd.amount,
        asset_type: asset.asset_type,
        confidential_amount: cmd.is_confidential_amount,
        keypair: secret.key.into_keypair(),
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
        // TODO: change back to ? when abcf error type is satisfying
        builder.build(&mut rng)?.to_bytes().unwrap(),
    )))?;

    asset.is_issued = true;
    asset.is_confidential_amount = cmd.is_confidential_amount;
    entry_asset::Assets::new(home)?.update(&asset)?;

    Ok(Box::new(display_asset::Display::new(
        display_asset::DisplayType::Issue,
        vec![(asset, None)],
    )))
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
    fn test_command_asset_execute_create() {
        let home = TempDir::new("test_command_asset_execute_create").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Create(Create {
                from_address: Some("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string()),
                from_secret: None,
                memo: None,
                is_transferable: false,
                decimal_place: 6,
                maximum: None,
                name: None,
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
                asset_type: "1TYZSwkxQI6-q49vgFsCOuXaOjaHbhtEV2GyDoPglUU=".to_string(),
                is_confidential_amount: false,
                amount: 999,
            }),
        };

        // because we did not setup the findorad server
        // should be connection refused error
        assert!(cmd.execute(home.path(), "127.0.0.1").is_err());
    }

    #[test]
    fn test_command_asset_execute_show() {
        let home = TempDir::new("test_command_asset_execute_show").unwrap();
        let cmd = Command {
            subcmd: SubCommand::Show(Show {
                from_address: Some("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string()),
                from_secret: None,
            }),
        };

        // because we did not setup the findorad server
        // should be connection refused error
        assert!(cmd.execute(home.path(), "127.0.0.1").is_err());
    }
}
