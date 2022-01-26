use std::fs;
use std::fs::File;
use serde_json::Value;

pub fn replace_config(abcf_path:&str, tcp_port: &str, p2p_port:&str){
    let config = fs::read_to_string(abcf_path.clone());
    if config.is_err() {
        panic!("{}",config.unwrap_err().to_string());
    }

    let config = config.unwrap();

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
    ];

    let target_cfg = [
        "index_all_keys = true",
        &format!("laddr = \"tcp://0.0.0.0:{}\"",tcp_port.clone()),
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
        &format!("laddr = \"tcp://0.0.0.0:{}\"",p2p_port.clone()),
    ];

    let config = orig_cfg
        .iter()
        .zip(target_cfg.iter())
        .fold(config, |acc, pair| acc.replace(pair.0, pair.1));

    let result = fs::write(abcf_path.clone(), config);

    if result.is_err() {
        panic!("{}",result.unwrap_err().to_string());
    }
}

pub fn read_validators_from_genesis(genesis_path:String) -> Value {
    let genesis_str = fs::read_to_string(genesis_path.clone()).unwrap();

    let genesis_json = serde_json::from_str::<Value>(&genesis_str).unwrap();

    let map = genesis_json.as_object().unwrap();

    let validators = map.get("validators").unwrap().as_array().unwrap();

    let validator = validators.get(0).unwrap().clone();

    validator
}

pub fn save_validators_to_genesis(genesis_path:String, validators: Value) {
    let genesis_str = fs::read_to_string(genesis_path.clone()).unwrap();

    let mut genesis_json = serde_json::from_str::<Value>(&genesis_str).unwrap();

    let map = genesis_json.as_object_mut().unwrap();

    let vs = map.get_mut("validators").unwrap();

    *vs = validators;


    let bytes = serde_json::to_vec_pretty(&genesis_json).unwrap();

    fs::write(genesis_path,bytes).unwrap();
}