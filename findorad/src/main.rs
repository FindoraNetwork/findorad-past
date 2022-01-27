#![feature(generic_associated_types)]

mod config;
mod error;
mod evm;
mod findorad;

pub use error::*;

mod command;

use crate::command::Action;
use clap::Parser;
use command::Args;

fn main() {
    env_logger::init();

    let args = Args::parse();

    if args.dev {
        let mut fnd = findorad::Findorad::new(None);
        let tx = command::dev::define_issue_fra();
        fnd.genesis(tx).unwrap();
        fnd.start();
    }

    if args.dev_staking {
        command::dev_staking::start();
    }

    if args.action.is_some() {
        match args.action.unwrap() {
            Action::Node(node) => {
                let mut fnd = findorad::Findorad::new(Some(node.config_path.as_str()));
                let tx = command::dev::define_issue_fra();
                fnd.genesis(tx).unwrap();
                fnd.start();
            }
        }
    }
}
