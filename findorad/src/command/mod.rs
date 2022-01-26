pub mod dev;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Start a development environment for single node.
    #[clap(short, long)]
    pub dev: bool,

    /// Start a development environment for staking.
    #[clap(short = 's', long)]
    pub dev_staking: bool,

    /// Enable web3 interface.
    #[clap(long)]
    pub enable_web3: bool,

    #[clap(subcommand)]
    pub action: Option<Action>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    Node,
}
