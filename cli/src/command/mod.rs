use clap::Clap;

mod setup;
mod batch;
mod execute;
mod transfer;
mod issue;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Fn {
    #[clap(short, long, env = "FN_HOME", default_value = "~/.findora/fn")]
    pub home: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap, Debug)]
enum SubCommand {
    #[clap(version, author, about)]
    Setup(setup::Command),
    #[clap(version, author, about)]
    Batch(batch::Command),
    #[clap(version, author, about)]
    Execute(execute::Command),
    #[clap(version, author, about)]
    Transfer(transfer::Command),
    #[clap(version, author, about)]
    Issue(issue::Command),
}

