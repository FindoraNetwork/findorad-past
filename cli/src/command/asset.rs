use clap::{ArgGroup, Parser};
use ruc::Result;

use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("asset"))]
pub struct Command {}

impl Command {
    pub async fn execute(&self, config: Config) -> ruc::Result<()> {
        Ok(())
    }
}
