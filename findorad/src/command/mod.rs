mod dev;

use clap::Parser;
use ruc::*;


#[derive(Parser, Debug)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

impl Opts {
    pub fn execute(&self) -> Result<()> {

        match &self.subcmd {
            SubCommand::Dev(c) => c.execute()?,
        }

        Ok(())
    }
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(version, author, about = "Setup configuration entry.")]
    Dev(dev::Command),
}