use clap::{ArgGroup, Parser};

use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("asset"))]
pub struct Command {}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        Ok(())
    }
}
