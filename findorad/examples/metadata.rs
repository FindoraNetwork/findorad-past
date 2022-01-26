use libfindora::Address;
use libfn::types::Wallet;

fn main() {
    env_logger::init();

    let wallet = Wallet::from_mnemonic("dentist earth learn way nominee satisfy scorpion curious gate chapter draw river broom tenant empower ordinary grunt window horn balance stone marble flat found").unwrap();
    let kp = wallet.secret.key.into_keypair();

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut p1 = abcf_sdk::providers::HttpGetProvider {
            url: String::from("http://127.0.0.1:26657"),
        };
        let address = Address::from(kp.pub_key);
        println!("{}", serde_json::to_string(&address).unwrap());
        let result = libfn::net::metadata::get(&mut p1).await.unwrap();
        println!("query: {:?}", result);
    });
}
