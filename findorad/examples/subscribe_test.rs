use abcf_sdk::jsonrpc::Request;
use tokio::runtime::Runtime;
use abcf_sdk::providers::{Provider, WsProvider};
use serde_json::json;
use serde_json::Value;
use ruc::*;

fn main() -> Result<()> {
    let rt = Runtime::new().unwrap();
    let mut provider = WsProvider::new();
    let query = json!(["tm.event='SendEvent'"]);
    let subscribe_req = Request::new_to_str("subscribe", query);
    let resp = provider.request("subscribe", &*subscribe_req).await?;

    println!("{:?}", resp);

    for _ in 0..10 {
        let r = provider.receive().await.unwrap().unwrap();
        println!("{:#?}", r);
    }

    Ok(())
}