use clap::Parser;

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Set the address of Findora rpc server
    set_server_address: Option<String>,
}

impl Command {
    pub async fn execute(&self, config: Config) -> ruc::Result<()> {
        let mut config = config;

        if let Some(addr) = &self.set_server_address {
            config.node.address = addr.clone();
            config.save()?;
        }

        Ok(())
    }
}
