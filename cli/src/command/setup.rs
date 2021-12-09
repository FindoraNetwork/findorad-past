use clap::{ArgGroup, Parser};
// use ruc::*;

use crate::config::Config;

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("server_address"))]
pub struct Command {
    #[clap(short = 's', long, group = "server_address")]
    /// Set findorad rpc address.
    set_server_address: Option<String>,

    #[clap(short = 'S', long, group = "server_address")]
    /// Get findorad rpc address.
    get_server_address: bool,
}

impl Command {
    pub async fn execute(&self, config: Config) -> ruc::Result<()> {
        let mut config = config;

        if self.get_server_address {
            println!("{}", config.node.address);
        }

        if let Some(addr) = &self.set_server_address {
            config.node.address = addr.clone();
            config.save()?;
        }

        Ok(())
    }
}
