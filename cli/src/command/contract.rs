use clap::Parser;

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Display the information of the contract account
    Account(Account),
    /// Transfer FRA from a Findora address to the specified Ethereum address
    Deposit(Deposit),
    /// Transfer FRA from an Ethereum address to the specified Findora address
    Withdraw(Withdraw),
}

#[derive(Parser, Debug)]
struct Account {
    /// Address of Findora(fra1rkv...) or Ethereum(0xd3Bf...)
    #[clap(forbid_empty_values = true, value_name = "ADDRESS")]
    addr: String,
}

#[derive(Parser, Debug)]
struct Deposit {
    /// Amount of FRA to deposit
    #[clap(forbid_empty_values = true)]
    amount: u64,
    /// Address of Ethereum(0xd3Bf...) to receive FRA
    #[clap(short, long, value_name = "ADDRESS")]
    addr: Option<String>,
    /// Findora private key for signing a deposit transaction
    #[clap(short, long)]
    private_key: Option<String>,
}

#[derive(Parser, Debug)]
struct Withdraw {
    /// Amount of FRA to withdrawal
    #[clap(forbid_empty_values = true)]
    amount: u64,
    /// Address of Findora(fra1rkv...) to receive FRA
    #[clap(short, long, value_name = "ADDRESS")]
    addr: Option<String>,
    /// Ethereum private key for signing a withdraw transaction
    #[clap(short, long)]
    private_key: Option<String>,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        match &self.subcmd {
            SubCommand::Account(_account) => {}
            SubCommand::Deposit(_deposit) => {}
            SubCommand::Withdraw(_withdraw) => {}
        }
        Ok(())
    }
}
