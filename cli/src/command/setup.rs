use clap::{Clap, ArgGroup};
use ruc::*;

use crate::config::Config;

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("server_address"))]
#[clap(group = ArgGroup::new("account"))]
pub struct Command {
    #[clap(short = 's', long, group = "server_address")]
    /// Set findorad rpc address.
    set_server_address: Option<String>,

    #[clap(short = 'S', long, group = "server_address")]
    /// Get findorad rpc address.
    get_server_address: bool,

    #[clap(short, long, group = "account")]
    /// Add account by mnemonic.
    add_mnemonic: Option<String>,

    #[clap(short, long, group = "account")]
    /// List account.
    list_account: bool,

    #[clap(short, long, group = "account")]
    /// List account.
    delete_account: Option<usize>,
}

impl Command {
    pub fn execute(&self, config: Config) -> Result<()> {
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
