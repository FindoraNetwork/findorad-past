#![feature(generic_associated_types)]

mod config;
mod error;
mod findorad;
mod web3;
pub use error::*;

mod command;

use crate::command::Action;
use clap::Parser;
use command::Args;

fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.enable_web3 {
        web3::strat_web3();
    }

    if args.dev {
        let mut fnd = findorad::Findorad::new(None);
        let tx = command::dev::define_issue_fra();
        fnd.genesis(tx).unwrap();
        fnd.start();
    }

    if args.dev_staking {
        command::dev_staking::start(None);
    }

    if args.action.is_some() {
        match args.action.unwrap() {
            Action::StakingNode(node) => {
                let mut fnd = findorad::Findorad::new(Some(node.config_path.as_str()));
                let tx = command::dev::define_issue_fra();
                fnd.genesis(tx).unwrap();
                fnd.start();
            }
        }
    }
}
