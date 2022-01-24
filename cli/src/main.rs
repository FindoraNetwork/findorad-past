use clap::Parser;
use console::{style, Emoji};

pub(crate) mod command;
pub(crate) mod config;
pub(crate) mod display;
pub(crate) mod entry;
pub(crate) mod utils;

fn main() {
    env_logger::init();
    let opts = command::Opts::parse();

    match opts.execute() {
        Ok(v) => {
            println!("{}", v)
        }
        Err(e) => {
            println!(
                "{} {}",
                Emoji("❌", "x "),
                style("Something Wrong...").bold().red()
            );
            for cause in e.chain() {
                eprintln!("{} {}", Emoji("🔥", "x "), style(cause).yellow());
            }
            std::process::exit(1);
        }
    }
}
