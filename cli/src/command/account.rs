use clap::{ArgGroup, Parser};

use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("account").required(true).args(&["addr"]))]
pub struct Command {
    /// Findora account(fra1rkv...) or Ethereum address(0xd3Bf...)
    #[clap(value_name = "ADDRESS")]
    addr: String,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        Ok(())
    }
}
