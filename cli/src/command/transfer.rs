// use std::future::poll_fn;
use std::{fmt::Display, path::Path};

use crate::entry::wallet as entry_wallet;

use anyhow::Result;
use clap::{ArgGroup, Parser};
use libfn::{entity, Builder};

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
    /// Show a list of saved batch process names or specific one for detail information
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
    #[clap(short = 'e', long, required = true, forbid_empty_values = true)]
    asset_type: String,
    /// Amount of the Asset Type to send
    #[clap(short = 'a', long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address to send which is a
    /// 1. ETH compatible address (0x...) or
    /// 2. Findora addreess (fra...)
    #[clap(short = 't', long, required = true, forbid_empty_values = true)]
    to_address: String,
    /// Make the amount confidential in the transaction
    #[clap(short = 'A', long)]
    is_confidential_amount: bool,
    /// Make the asset code confidential in the transaction
    #[clap(short = 'T', long)]
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
    #[clap(short, long, forbid_empty_values = true)]
    batch_name: Option<String>,
}

impl Command {
    pub fn execute(&self, home: &Path) -> Result<Box<dyn Display>> {
        let wallets = entry_wallet::Wallets::new(home)?;
        match &self.subcmd {
            SubCommand::Send(cmd) => send(cmd, &wallets),
            SubCommand::Save(cmd) => save(cmd),
            SubCommand::Batch(cmd) => batch(cmd),
            SubCommand::Show(cmd) => show(cmd),
        }
    }
}

fn send(cmd: &Send, wallets: &entry_wallet::Wallets) -> Result<Box<dyn Display>> {
    let wallet = if let Some(addr) = &cmd.from_address {
        wallets.read().from_address(addr).build()?
    } else if let Some(secret) = &cmd.from_secret {
        wallets.read().from_secret(secret).build()?
    } else {
        // since the clap will check the input cannot be empty by atribute
        // forbid_empty_values = true
        unreachable!()
    };

    let t = entity::Transfer::builder()
        .from(&wallet.address)
        .public_key(&wallet.public)
        .address(&entry_wallet::detect_address(&cmd.to_address)?)
        .amount(cmd.amount)
        .asset_type(&cmd.asset_type)
        .confidential_amount(cmd.is_confidential_amount)
        .confidential_asset(cmd.is_confidential_asset)
        .build()?;

    let mut prng = ChaChaRng::from_entropy();
    let mut provider = HttpGetProvider {};
    let mut builder = Builder::default();

    block_on(builder.from_entities(&mut prng, &mut provider, vec![entity::Entity::Transfer(t)]))?;
    let tx = builder.build(&mut prng)?;
    tx.serialize()?;
    Ok(Box::new(0))
}

fn save(_cmd: &Save) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}

fn batch(_cmd: &Batch) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}

fn show(_cmd: &Show) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}
