use ruc::*;
use libfindora::transaction::{Transaction};
use findorad_lib::utxo_module_rpc;
use tokio::runtime::Runtime;

pub fn send_tx(tx: &Transaction) -> Result<()>{

    let tx_capnp = tx.to_bytes()?;

    let tx_hex = hex::encode(&tx_capnp);
    println!("tx hex:{:?}",tx_hex);

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let provider = abcf_sdk::providers::HttpGetProvider{};
        let r = utxo_module_rpc::send_tx(tx_hex,"broadcast_tx_sync",provider).await;
        println!("resp: {:?}",r);
    });

    Ok(())
}

