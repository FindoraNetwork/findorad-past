use crate::{command, findorad::Findorad};
use abcf_node::NodeType;
use serde_json::{json, Value};
use std::{
    fs::{self, File},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn current_dir() -> PathBuf {
    let s = std::env::current_exe().unwrap();
    s.parent().unwrap().to_path_buf()
}

pub fn start(path: Option<&Path>) {
    let ports = vec![
        (26616, 26615),
        (26626, 26625),
        (26636, 26635),
        (26646, 26645),
        (26654, 26655),
        (26666, 26665),
        (26676, 26675),
        (26686, 26685),
        (26696, 26695),
        (26706, 26705),
        (26716, 26715),
        (26726, 26725),
        (26746, 26745),
        (26756, 26755),
        (26766, 26765),
        (26776, 26775),
        (26786, 26785),
        (26796, 26795),
        (26806, 26805),
        (26816, 26815),
    ];

    // findorad execute path
    let work_dir = if let Some(p) = path {
        p.to_path_buf()
    } else {
        current_dir()
    };

    // Create Node 00.
    let node0_path = work_dir.join("nodes").join("node00");

    let mut fnd = Findorad::new(node0_path.to_str());

    // TODO: If exists, don't genesis;
    let tx = command::dev::define_issue_fra();
    fnd.genesis(tx).unwrap();
    fnd.flush().unwrap();

    let nodeid = read_node_key(&node0_path);
    let len = ports.len();

    // Create Node 01 .. 20
    let validator_keys = create_node(&work_dir, &nodeid, ports);

    let node00_genesis_path = node0_path.join("abcf/config/genesis.json");
    save_validators_and_chain_id_to_genesis(&node00_genesis_path, validator_keys);

    for i in 1..len {
        let path = work_dir
            .join("nodes")
            .join(format!("node{:02}", i))
            .join("abcf/config/genesis.json");
        fs::copy(&node00_genesis_path, path).unwrap();
    }

    start_node(&work_dir, len);

    fnd.start();
}

fn start_node(path: &Path, len: usize) {
    for i in 1..len {
        let node_path = path.join("nodes").join(format!("node{:02}", i));

        let output = node_path.join("output.log");
        let err = node_path.join("error.log");

        let of = File::create(output).unwrap();
        let ef = File::create(err).unwrap();

        Command::new(std::env::current_exe().unwrap().to_str().unwrap())
            .args(["node", "-c", node_path.to_str().unwrap()])
            .stdout(Stdio::from(of))
            .stderr(Stdio::from(ef))
            .spawn()
            .unwrap();
    }
}

fn create_node(p: &Path, nodeid: &str, ports: Vec<(u32, u32)>) -> Vec<Value> {
    let node_path = p.join("nodes").join("node00");
    let priv_key_path = node_path.join("abcf/config/priv_validator_key.json");
    let validator_address = read_validator_address(&priv_key_path, "node00".to_string());

    let mut validator_keys = vec![validator_address];

    let persistent_peers = format!("{}@127.0.0.1:26656", nodeid);

    for (i, (rpc_port, p2p_port)) in ports.iter().enumerate() {
        let node_name = format!("node{:02}", i + 1);
        let node_path = p.join("nodes").join(node_name.as_str());

        tendermint_sys::init_home(
            node_path.join("abcf").to_str().unwrap(),
            NodeType::Validator,
        )
        .unwrap();

        let config_path = node_path.join("abcf/config/config.toml");

        replace_config(&config_path, *rpc_port, *p2p_port, &persistent_peers);

        let priv_key_path = node_path.join("abcf/config/priv_validator_key.json");
        let validator_address = read_validator_address(&priv_key_path, node_name);

        validator_keys.push(validator_address);
    }

    validator_keys
}

fn read_validator_address(path: &Path, node_name: String) -> Value {
    let content = fs::read_to_string(path).unwrap();
    let obj = serde_json::from_str::<Value>(&content).unwrap();
    let address = obj.get("address").unwrap().clone();
    let pubkey = obj.get("pub_key").unwrap().clone();

    let mut map = serde_json::Map::new();

    map.insert("address".to_string(), address);
    map.insert("pub_key".to_string(), pubkey);
    map.insert("power".to_string(), Value::String("1".to_string()));
    map.insert("name".to_string(), Value::String(node_name));

    Value::Object(map)
}

fn replace_config(abcf_path: &Path, rpc_port: u32, p2p_port: u32, persistent_peers: &str) {
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

fn save_validators_and_chain_id_to_genesis(genesis_path: &Path, validators: Vec<Value>) {
    let genesis_str = fs::read_to_string(genesis_path).unwrap();

    let mut genesis_json = serde_json::from_str::<Value>(&genesis_str).unwrap();

    let map = genesis_json.as_object_mut().unwrap();

    map.insert("validators".to_string(), Value::Array(validators));
    map.insert("chain_id".to_string(), json!("findorad-dev-staking"));

    let bytes = serde_json::to_vec_pretty(&genesis_json).unwrap();

    fs::write(genesis_path, bytes).unwrap();
}

fn read_node_key(path: &Path) -> String {
    let node_key_path = path.join("abcf/config/node_key.json");
    let node_key_str = fs::read_to_string(node_key_path).unwrap();
    let node_key_json = serde_json::from_str::<Value>(&node_key_str).unwrap();
    let mut id = node_key_json.get("id").unwrap().to_string();
    id.remove(0);
    id.pop();
    id
}
