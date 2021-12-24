use std::fmt::Display;

use crate::config::Config;

use anyhow::Result;
use clap::{ArgGroup, Parser};

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
    /// Amount of FRA tokens to send
    #[clap(short, long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address to send which is a
    /// 1. ETH compatible address (0x...) or
    /// 2. Findora addreess (fra...)
    #[clap(short, long, required = true, forbid_empty_values = true)]
    to_address: String,
    /// Make the amount confidential in the transaction
    #[clap(short = 'A', long)]
    confidential_amount: bool,
    /// Make the asset code confidential in the transaction
    #[clap(short = 'T', long)]
    confidential_asset: bool,
}

#[derive(Parser, Debug)]
struct Save {
    /// Name of the batch process for identifying in the batch command
    /// Save with the same batch name will appending the new request
    #[clap(short, long, required = true, forbid_empty_values = true)]
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
    pub fn execute(&self, _config: Config) -> Result<Box<dyn Display>> {
        match &self.subcmd {
            SubCommand::Send(cmd) => send(cmd),
            SubCommand::Save(cmd) => save(cmd),
            SubCommand::Batch(cmd) => batch(cmd),
            SubCommand::Show(cmd) => show(cmd),
        }
    }
}

fn send(cmd: &Send) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}

fn save(cmd: &Save) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}

fn batch(cmd: &Batch) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}

fn show(cmd: &Show) -> Result<Box<dyn Display>> {
    Ok(Box::new(0))
}
