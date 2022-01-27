use crate::{command, findorad};
use abcf_node::NodeType;
use serde_json::{json, Value};
use std::env::current_exe;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn start() {
    let ports = vec![
        ("26616", "26615"),
        ("26626", "26625"),
        ("26636", "26635"),
        ("26646", "26645"),
        ("26654", "26655"),
        ("26666", "26665"),
        ("26676", "26675"),
        ("26686", "26685"),
        ("26696", "26695"),
        ("26706", "26705"),
        ("26716", "26715"),
        ("26726", "26725"),
        ("26746", "26745"),
        ("26756", "26755"),
        ("26766", "26765"),
        ("26776", "26775"),
        ("26786", "26785"),
        ("26796", "26795"),
        ("26806", "26805"),
        ("26816", "26815"),
    ];

    // ./target/debug/findorad
    let cur_path = current_exe().unwrap();
    // ./target
    let grand_path = cur_path.parent().unwrap().parent().unwrap();
    // ./target/node/node{}/findorad
    let node_paths = create_findorad_paths(grand_path);
    // ./target/node/node{}/findorad/abcf/config/config.toml
    let config_paths = create_config_paths(&node_paths);
    // ./target/node/node{}/findorad/abcf/config/genesis.json
    let genesis_paths = create_genesis_paths(&node_paths);

    let mut validators = vec![];

    let mut fnd = findorad::Findorad::new(None);

    // create config
    for (node, genesis) in node_paths.iter().zip(&genesis_paths) {
        //create tendermint file
        let p = node.join("abcf");
        tendermint_sys::init_home(p.to_str().unwrap(), NodeType::Validator).unwrap();

        //get validator
        let validator = read_validators_from_genesis(genesis.as_path());
        validators.push(validator);
    }

    let config_26657 = grand_path.join("findorad").join("abcf").join("config");

    let node_key_26657 = config_26657.join("node_key.json");

    //get 26657 id
    let id = read_node_key(node_key_26657.as_path());
    let persistent_peers = format!("{}@127.0.0.1:26656", id);

    let validators_value = serde_json::to_value(validators).unwrap();
    let genesis_26657 = config_26657.join("genesis.json");

    //set validators and chainId
    save_validators_and_chain_id_to_genesis(genesis_26657.as_path(), validators_value);

    //copy to other node genesis
    for genesis in genesis_paths {
        let p = genesis.as_path();
        copy_genesis(p, genesis_26657.as_path());
    }

    //replace config
    for (config, (rpc, p2p)) in config_paths.iter().zip(ports) {
        replace_config(config, rpc, p2p, persistent_peers.as_str());
    }

    start_node(node_paths, cur_path.as_path());

    let tx = command::dev::define_issue_fra();
    fnd.genesis(tx).unwrap();
    fnd.start();
}

fn start_node(node_paths: Vec<PathBuf>, cur_path: &Path) {
    for node_path in node_paths {
        let output = node_path.join("output.log");
        let f = File::create(output).unwrap();
        Command::new(cur_path.to_str().unwrap())
            .args(["node", "-c", node_path.to_str().unwrap()])
            .stdout(Stdio::from(f))
            .spawn()
            .unwrap();
    }
}

fn create_findorad_paths(p: &Path) -> Vec<PathBuf> {
    let mut v = vec![];

    for i in 0..20 {
        let node_path = p.join("node").join(format!("node{}", i)).join("findorad");
        v.push(node_path);
    }

    //add 26657 node
    v.push(p.join("findorad"));

    v
}

fn create_config_paths(findorads: &[PathBuf]) -> Vec<PathBuf> {
    let mut v = vec![];

    for findorad in findorads {
        let config = findorad
            .clone()
            .join("abcf")
            .join("config")
            .join("config.toml");
        v.push(config);
    }

    v
}

fn create_genesis_paths(findorads: &[PathBuf]) -> Vec<PathBuf> {
    let mut v = vec![];

    for findorad in findorads {
        let genesis = findorad
            .clone()
            .join("abcf")
            .join("config")
            .join("genesis.json");
        v.push(genesis);
    }

    v
}

fn replace_config(abcf_path: &Path, rpc_port: &str, p2p_port: &str, persistent_peers: &str) {
    let config = fs::read_to_string(abcf_path).unwrap();

    let orig_cfg = [
        "index_all_keys = false",
        "laddr = \"tcp://127.0.0.1:26657\"",
        "timeout_propose = \"3s\"",
        "timeout_propose_delta = \"500ms\"",
        "timeout_prevote = \"1s\"",
        "timeout_prevote_delta = \"500ms\"",
        "timeout_precommit = \"1s\"",
        "timeout_precommit_delta = \"500ms\"",
        "timeout_commit = \"1s\"",
        "recheck = true",
        "fast_sync = true",
        "size = 5000",
        "prometheus = false",
        "laddr = \"tcp://0.0.0.0:26656\"",
        "persistent-peers = \"\"",
    ];

    let target_cfg = [
        "index_all_keys = true",
        &format!("laddr = \"tcp://0.0.0.0:{}\"", rpc_port),
        "timeout_propose = \"8s\"",
        "timeout_propose_delta = \"100ms\"",
        "timeout_prevote = \"4s\"",
        "timeout_prevote_delta = \"100ms\"",
        "timeout_precommit = \"4s\"",
        "timeout_precommit_delta = \"100ms\"",
        "timeout_commit = \"15s\"",
        "recheck = false",
        "fast_sync = false",
        "size = 2000",
        "prometheus = false",
        &format!("laddr = \"tcp://0.0.0.0:{}\"", p2p_port),
        &format!("persistent-peers = \"{}\"", persistent_peers),
    ];

    let config = orig_cfg
        .iter()
        .zip(target_cfg.iter())
        .fold(config, |acc, pair| acc.replace(pair.0, pair.1));

    fs::write(abcf_path, config).unwrap();
}

fn read_validators_from_genesis(genesis_path: &Path) -> Value {
    let genesis_str = fs::read_to_string(genesis_path).unwrap();

    let genesis_json = serde_json::from_str::<Value>(&genesis_str).unwrap();

    let map = genesis_json.as_object().unwrap();

    let validators = map.get("validators").unwrap().as_array().unwrap();

    let validator = validators.get(0).unwrap().clone();

    validator
}

fn save_validators_and_chain_id_to_genesis(genesis_path: &Path, validators: Value) {
    let genesis_str = fs::read_to_string(genesis_path).unwrap();

    let mut genesis_json = serde_json::from_str::<Value>(&genesis_str).unwrap();

    let map = genesis_json.as_object_mut().unwrap();

    map.insert("validators".to_string(), validators);
    map.insert("chain_id".to_string(), json!("findorad"));

    let bytes = serde_json::to_vec_pretty(&genesis_json).unwrap();

    fs::write(genesis_path, bytes).unwrap();
}

fn copy_genesis(to_genesis_path: &Path, from_genesis_path: &Path) {
    let genesis_bytes = fs::read(from_genesis_path).unwrap();
    fs::write(to_genesis_path, genesis_bytes).unwrap();
}

fn read_node_key(node_key_path: &Path) -> String {
    let node_key_str = fs::read_to_string(node_key_path).unwrap();
    let node_key_json = serde_json::from_str::<Value>(&node_key_str).unwrap();
    let mut id = node_key_json.get("id").unwrap().to_string();
    id.remove(0);
    id.pop();
    id
}
