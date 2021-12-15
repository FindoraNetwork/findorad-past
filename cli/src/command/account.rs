use clap::Parser;

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    /// Findora address(fra1rkv...) or Ethereum address(0xd3Bf...)
    #[clap(value_name = "ADDRESS")]
    addr: String,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        Ok(())
    }
}
