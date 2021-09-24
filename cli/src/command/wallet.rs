use clap::Clap;
use ruc::*;

use crate::config::Config;

#[derive(Clap, Debug)]
pub struct Command {
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
    pub fn execute(&self, _config: Config) -> Result<()> {
        Ok(())
    }
}
