use clap::{Parser, ValueHint};

use crate::config::Config;

use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Stake the validator node of yourself
    StakeSelf(StakeSelf),
    /// Stake the validator node of others
    StakeOther(StakeOther),
    /// Claim accumulated FRA rewards from a validator node
    Claim(Claim),
    /// Show the validator status and accumulated rewards
    Show(Show),
}

#[derive(Parser, Debug)]
struct StakeData {
    /// File path to the Findora wallet which contains base64-formatted XfrPrivateKey
    #[clap(short, long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    secret_key: Option<PathBuf>,
    /// Amount of FRA tokens to stake
    #[clap(long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address(1DE3EED...) of the validator node to stake
    #[clap(long, required = true, forbid_empty_values = true)]
    address: String,
}

#[derive(Parser, Debug)]
struct StakeSelf {
    #[clap(flatten)]
    data: StakeData,
    /// Description of the validator node
    #[clap(short, long)]
    description: Option<String>,
    /// Commission rate of the validator node
    #[clap(long, required = true, forbid_empty_values = true)]
    commission_rate: f64,
}
#[derive(Parser, Debug)]
struct StakeOther {
    #[clap(flatten)]
    data: StakeData,
}

#[derive(Parser, Debug)]
pub struct Claim {
    /// File path to the Findora wallet which contains base64-formatted XfrPrivateKey
    #[clap(short, long, value_name = "PATH", value_hint = ValueHint::FilePath)]
    secret_key: Option<PathBuf>,
    /// Amount of FRA tokens to claim
    #[clap(long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address(1DE3EED...) of the validator node to claim
    #[clap(long, required = true, forbid_empty_values = true)]
    address: String,
}

#[derive(Parser, Debug)]
struct Show {
    /// Address(1DE3EED...) of the validator node to show
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        match &self.subcmd {
            SubCommand::StakeSelf(_stake_self) => {}
            SubCommand::StakeOther(_stake_other) => {}
            SubCommand::Claim(_claim) => {}
            SubCommand::Show(_show) => {}
        }
        Ok(())
    }
}
