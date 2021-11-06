use abcf_sdk::jsonrpc::Request;
use abcf_sdk::providers::{Provider, WsProvider};
use ruc::*;
use serde_json::json;
use serde_json::Value;
use std::thread::park;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    let rt = Runtime::new().unwrap();
    let mut provider = WsProvider::new();
    // pub_key can be change from test case
    let query = json!(["SendEvent.pub_key='DK5o6w6OkXk6soHvMToYfp0W/rIWuk9ODjukNEpUFKI='"]);
    let subscribe_req = Request::new_to_value("subscribe", query);

    rt.block_on(async {
        let resp = provider
            .request::<Value, String>("subscribe", &subscribe_req)
            .await
            .unwrap();
        println!("{:?}", resp);

        loop {
            let r = provider.receive().await.unwrap();
            println!("{:?}", r);
        }
    });

    park();

    Ok(())
}
