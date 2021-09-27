fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    // let path = env!("OUT_DIR").to_string();
    let rpc_module = out_dir.clone() + "/utxomodule.rs";
    let coinbase_module = out_dir.clone() + "/coinbasemodule.rs";

    std::fs::File::create(rpc_module).unwrap();
    std::fs::File::create(coinbase_module).unwrap();

}
