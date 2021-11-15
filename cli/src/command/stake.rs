use clap::{ArgGroup, Parser};
use ruc::*;

use crate::config::Config;
use crate::utils::query_validators;

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

    #[clap(short = 'D', long, group = "operation")]
    /// delegation
    delegation: bool,

    #[clap(short = 'U', long, group = "operation")]
    /// delegation
    undelegation: bool,

    #[clap(long)]
    amount: u64
}

impl Command {
    pub async fn execute(&self, _config: Config) -> Result<()> {

        if self.list_validators {
            query_validators();
            return Ok(())
        }

        if self.get_stake_info {

            if let Some(sk) = &self.from_secret_key {

            }

            if let Some(account) = self.account {

            }

        }

        if self.delegation {

            let pk = if let Some(sk) = &self.from_secret_key {

            } else if let Some(account) = self.account {

            };


        }

        if self.undelegation {

        }


        Ok(())
    }
}
