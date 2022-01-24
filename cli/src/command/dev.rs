use abcf::ToBytes;
use libfindora::asset::FRA;
use libfn::entity::Entity::{Define as EDefine, Issue as EIssue};
use libfn::entity::{Define, Issue};
use libfn::Builder;
use primitive_types::U256;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use anyhow::Result;
use clap::Parser;
use libfindora::Address;
use libfn::types::Wallet;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// define and issue fra
    DIFra(DIFra),
}

#[derive(Parser, Debug)]
struct DIFra {}

impl Command {
    pub fn execute(&self) -> Result<Box<dyn Display>> {
        match &self.subcmd {
            SubCommand::DIFra(_) => define_issue_fra(),
        }
    }
}

pub struct EmptyP {}

#[async_trait::async_trait]
impl abcf_sdk::providers::Provider for EmptyP {
    async fn request<Req, Resp>(
        &mut self,
        _method: &str,
        _params: &Req,
    ) -> abcf_sdk::error::Result<Option<Resp>>
    where
        Req: Serialize + Send + Sync,
        Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        Ok(None)
    }

    async fn receive(&mut self) -> abcf_sdk::error::Result<Option<String>> {
        Ok(None)
    }
}

fn define_issue_fra() -> Result<Box<dyn Display>> {
    // ETH Compatible Address: 0x75fc8ac096a993c48ce9b50e283590e19dee343e
    // Findora Address:        fra11wh7g4syk4xfufr8fk58zsdvsuxw7udp7pr5gn8
    // Findora Public Key:     7C2budB1QtXfoRJ-g-GN4BMdXpgkLv7MtTXG2yC3K3Q=
    // Secret:                 _12euPXJxDbpcw7fMNJufUZgrTgcK7ShTJmXuZZe8eM=
    // Mnemonic:
    // dentist earth learn way nominee satisfy scorpion curious gate chapter draw river broom tenant empower ordinary grunt window horn balance stone marble flat found

    let server_url = "http://127.0.0.1:26657";

    let wallet = Wallet::from_mnemonic("dentist earth learn way nominee satisfy scorpion curious gate chapter draw river broom tenant empower ordinary grunt window horn balance stone marble flat found").unwrap();
    let kp = wallet.secret.key.into_keypair();

    let define_entry = EDefine(Define {
        maximum: Some(U256::from(1000 + 210_0000_0000 * FRA.units)),
        transferable: true,
        keypair: kp.clone(),
        asset: FRA.bare_asset_type,
    });

    let issue_entry = EIssue(Issue {
        amount: 210_0000_0000 * FRA.units / 2,
        asset_type: FRA.bare_asset_type,
        confidential_amount: false,
        keypair: kp.clone(),
    });

    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut flag = false;

    rt.block_on(async {
        let mut p = EmptyP {};
        let mut p1 = abcf_sdk::providers::HttpGetProvider {
            url: server_url.to_string(),
        };
        let mut prng = ChaChaRng::from_entropy();
        let mut builder = Builder::default();
        builder
            .from_entities(&mut prng, &mut p, vec![define_entry, issue_entry])
            .await
            .unwrap();
        let tx = builder.build(&mut prng).unwrap();

        let tx = tx.to_bytes().unwrap();
        let result = libfn::_send_tx(&mut p1, tx).await.unwrap();
        if let Some(res) = result {
            println!(
                "code:{:?}, msg:{:?}, codespace:{:?}",
                res.code, res.log, res.codespace
            );
            if res.code == 0 {
                flag = true;
            }
        } else {
            println!("send tx response none");
        }
    });

    if !flag {
        return Ok(Box::new("failed!"));
    }

    std::thread::sleep(std::time::Duration::from_secs(5));

    rt.block_on(async {
        let mut p1 = abcf_sdk::providers::HttpGetProvider {
            url: server_url.to_string(),
        };
        let address = Address::from(kp.pub_key);
        let result = libfn::get_owned_outputs(&mut p1, &address).await.unwrap();
        println!("address: {:?}", address);
        println!("owned_outputs: {:?}", result);
    });

    Ok(Box::new("success!"))
}
