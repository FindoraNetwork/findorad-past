use abcf_sdk::providers::Provider;
use libfindora::transaction::Transaction;
use ruc::*;
use serde_json::{json, Value};
use tokio::runtime::Runtime;

pub async fn send_tx_raw<P: Provider>(
    hex_str: &str,
    method: &str,
    mut p: P,
) -> Result<Option<Value>> {
    {
        let j = json!({ "tx": hex_str });
        let data = j.to_string();
        let resp = p.request(&method, &data).await.map_err(|_e| {
            println!("{:?}", _e);
            eg!()
        })?;

        println!("{:?}", resp);

        return if let Some(val) = resp {
            {
                let json = serde_json::from_str::<Value>(&val).c(d!())?;
                Ok(Some(json))
            }
        } else {
            {
                Ok(None)
            }
        };
    }
}

pub fn send_tx(tx: &Transaction) -> Result<()> {
    let tx_capnp = tx.to_bytes()?;

    let tx_hex = String::from("0x") + &hex::encode(&tx_capnp);

    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let provider = abcf_sdk::providers::HttpGetProvider {};
        let r = send_tx_raw(&tx_hex, "broadcast_tx_sync", provider).await;
        println!("resp: {:?}", r);
    });

    Ok(())
}
