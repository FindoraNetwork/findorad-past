pub mod dev;
pub mod dev_staking;

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

    #[clap(subcommand)]
    pub action: Option<Action>,
}

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    Node(Node),
}

#[derive(Parser, Debug)]
pub struct Node {
    #[clap(short, long, forbid_empty_values = true)]
    pub config_path: String,
}
