use clap::Parser;

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    /// Amount of the FRA tokens to claim [default: all]
    #[clap(short, long)]
    amount: Option<u64>,
    /// File path of Findora wallet which contains base64-formatted XfrPrivateKey
    #[clap(value_name = "FILE")]
    secret_key: std::path::PathBuf,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        Ok(())
    }
}
