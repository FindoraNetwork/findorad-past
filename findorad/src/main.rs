#![feature(generic_associated_types)]

mod config;
mod error;
mod evm;
mod findorad;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
pub use error::*;
use std::process::{Command, Stdio};
use abcf_node::NodeType;

mod command;
mod utils;

use clap::Parser;
use command::Args;
use crate::command::Action;
use crate::utils::{read_validators_from_genesis, replace_config, save_validators_to_genesis};

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
        let port_vec = vec![
            ("26616", "26615"), ("26626", "26625"), ("26636", "26635"), ("26646", "26645"),
            ("26654", "26655"), ("26666", "26665"), ("26676", "26675"), ("26686", "26685"),
            ("26696", "26695"), ("26706", "26705"), ("26716", "26715"), ("26726", "26725"),
            ("26746", "26745"), ("26756", "26755"), ("26766", "26765"), ("26776", "26775"),
            ("26786", "26785"), ("26796", "26795"), ("26806", "26805"), ("26816", "26815"),
        ];

        let mut genesis_vec = vec![];
        let mut validators = vec![];
        let mut prefix_path_vec = vec![];

        for (i, (tcp_port, p2p_port)) in port_vec.into_iter().enumerate() {
            let prefix_path = format!("./target/node/node{}/findorad",i);
            prefix_path_vec.push(prefix_path.clone());
            tendermint_sys::init_home(format!("{}/abcf",prefix_path).as_str(), NodeType::Validator).unwrap();
            replace_config(format!("{}/abcf/config/config.toml",prefix_path).as_str(), tcp_port, p2p_port);

            let genesis_path = format!("{}/abcf/config/genesis.json",prefix_path);
            let validator = read_validators_from_genesis(genesis_path.clone());
            
            validators.push(validator);
            genesis_vec.push(genesis_path.clone());

        }
        let mut fnd = findorad::Findorad::new(None);
        let validator = read_validators_from_genesis("./target/findorad/abcf/config/genesis.json".to_string());
        validators.push(validator);
        let value = serde_json::to_value(validators).unwrap();

        for genesis_path in genesis_vec {
            save_validators_to_genesis(genesis_path, value.clone());
        }
        save_validators_to_genesis("./target/findorad/abcf/config/genesis.json".to_string(), value.clone());

        for prefix_path in prefix_path_vec {
            let output = prefix_path.clone() + "/output.log";
            let f = File::create(output).unwrap();
            Command::new("cargo")
                .args(["run","--bin","findorad","--","node","-c",prefix_path.as_str()])
                .stdout(Stdio::from(f))
                .spawn()
                .unwrap();
        }

        let tx = command::dev::define_issue_fra();
        fnd.genesis(tx).unwrap();
        fnd.start();
    }

    if args.action.is_some() {
        match args.action.unwrap() {
            Action::Node(node) => {
                let fnd = findorad::Findorad::new(Some(node.config_path.as_str()));
                fnd.start();
            }
        }
    }
}
