use clap::{ArgGroup, Parser};
use ruc::*;

use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("operation"))]
#[clap(group = ArgGroup::new("account_info"))]
pub struct Command {
    #[clap(short = 'L', long, group = "operation")]
    /// List all available validator node's info.
    list_validators: bool,

    #[clap(short = 'S', long, group = "operation")]
    /// Get findorad rpc address.
    get_stake_info: bool,

    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short = 'f', long, group = "account_info")]
    /// From secret key.
    from_secret_key: Option<String>,

    #[clap(short, long, group = "account_info")]
    account: Option<usize>,

    #[clap(short, long)]
    validator: Option<String>,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> Result<()> {
        // let mut config = config;

        Ok(())
    }
}
