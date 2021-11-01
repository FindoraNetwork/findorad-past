use crate::config::Config;
use clap::Parser;
use ruc::*;

// use crate::utils::query_tx;

#[derive(Parser, Debug)]
pub struct Command {
    hash: Option<String>,
}

impl Command {
    pub async fn _execute(&self, _config: Config) -> Result<()> {
        // if let Some(hash) = &self.hash {
        //     query_tx(hash).await?;
        // }

        Ok(())
    }
}
