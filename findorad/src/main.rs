#![feature(generic_associated_types)]

mod config;
mod error;
mod findorad;
mod web3;
pub use error::*;

mod command;

use clap::Parser;
use command::Args;

fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.enable_web3 {
        web3::strat_web3();
    }

    if args.dev {
        let mut fnd = findorad::Findorad::new();
        let tx = command::dev::define_issue_fra();
        fnd.genesis(tx).unwrap();
        fnd.start();
    }
}
