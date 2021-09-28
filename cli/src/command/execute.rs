use clap::Clap;
use ruc::*;

use crate::config::Config;

#[derive(Clap, Debug)]
pub struct Command {
    /// Name of batch.
    batch_name: String,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}
