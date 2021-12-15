use clap::Parser;

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    typ: KeyType,
}

#[derive(Parser, Debug)]
enum KeyType {
    /// Generate a random Findora public and private key pair
    Findora,
    /// Generate an Ethereum public and private key pair from??? and memo too???
    Ethereum,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        match &self.typ {
            KeyType::Findora => {}
            KeyType::Ethereum => {}
        }
        Ok(())
    }
}
