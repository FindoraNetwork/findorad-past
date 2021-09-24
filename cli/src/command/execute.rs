use clap::Clap;
use ruc::*;

use crate::config::Config;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Findorad rpc address.
    id: String,
}

impl Command {
    pub fn execute(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}