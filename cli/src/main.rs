use clap::Parser;

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod entry;
pub(crate) mod utils;

fn main() {
    let opts = command::Opts::parse();

    match opts.execute() {
        Ok(v) => {
            println!("{}", v)
        }
        Err(e) => {
            panic!("{}", e);
        }
    }
}
