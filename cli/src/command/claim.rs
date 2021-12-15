use clap::{ArgGroup, Parser};

use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("claim"))]
pub struct Command {
    /// how many FRA units to claim [ default: all ]
    #[clap(short, long)]
    amount: Option<u64>,
    /// the file path which contains base64-formatted XfrPrivateKey of an existing wallet
    #[clap(value_name = "PATH")]
    private_key: std::path::PathBuf,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        Ok(())
    }
}
