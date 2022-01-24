#![feature(generic_associated_types)]

mod config;
mod error;
mod evm;
mod findorad;
pub use error::*;

mod command;

use clap::Parser;
use command::Args;

fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.dev {
        let mut fnd = findorad::Findorad::new();
        let tx = command::dev::define_issue_fra();
        fnd.genesis(tx).unwrap();
        fnd.start();
        return;
    }
}
