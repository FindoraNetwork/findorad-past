use crate::config::Config;
use clap::Clap;
use ruc::*;

use crate::utils::query_tx;

#[derive(Clap, Debug)]
pub struct Command {
    hash: Option<String>,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> Result<()> {

        if let Some(hash) = &self.hash {
            query_tx(hash).await?;
        }

        Ok(())
    }
}