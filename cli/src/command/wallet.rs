use std::collections::BTreeMap;

use abcf_sdk::providers::HttpGetProvider;
use clap::{ArgGroup, Clap};
use findorad_lib::utxo_module_rpc::get_owned_outputs;
use libfindora::utxo::GetOwnedUtxoReq;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;
use zei::{serialization::ZeiFromToBytes, xfr::{asset_record::open_blind_asset_record, sig::{XfrKeyPair, XfrSecretKey}}};

use crate::config::Config;

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("account"))]
pub struct Command {
    #[clap(short, long, group = "account")]
    /// Add account by mnemonic.
    add_mnemonic: Option<String>,

    #[clap(short, long, group = "account")]
    /// List account.
    list_account: bool,

    #[clap(short, long, group = "account")]
    /// List account.
    delete_account: Option<usize>,

    #[clap(short, long, group = "account")]
    /// List account.
    generate: bool,

    #[clap(short, long, group = "account")]
    /// Show account info.
    show: bool,

    wallet: Option<String>,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> Result<()> {
        if self.generate {
            let mut prng = ChaChaRng::from_entropy();
            let keypair = XfrKeyPair::generate(&mut prng);

            let pk_bytes = keypair.get_pk().zei_to_bytes();
            let pk_bytes_64 = base64::encode(&pk_bytes);

            println!("Public key is: {}", pk_bytes_64);

            let sk_bytes = keypair.get_sk().zei_to_bytes();
            let sk_bytes_64 = base64::encode(&sk_bytes);

            println!("Secret key is: {}", sk_bytes_64);
        }

        if self.show {
            let wallets = if let Some(w) = &self.wallet {
                let sk_bytes = base64::decode(&w).c(d!())?;
                let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;

                let keypair = sk.into_keypair();
                vec![keypair]
            } else {
                // TODO: Read wallet from home.
                Vec::new()
            };

            let params = GetOwnedUtxoReq {
                owners: wallets.iter().map(|kp| kp.get_pk()).collect(),
            };

            let provider = HttpGetProvider {};

            let result = get_owned_outputs(provider, params).await.unwrap();

            let mut value_map = BTreeMap::new();

            for oai in result.data.c(d!())?.outputs {
                let keypair = &wallets[oai.0];
                let output = oai.1.output;

                log::debug!("{:?}", output);

                let open_asset_record = open_blind_asset_record(&output.core, &output.owner_memo, keypair)?;

                log::debug!("Open Asset Recore is:{:?}", open_asset_record);

                let amount = open_asset_record.amount;
                let asset_type = open_asset_record.asset_type;

                if let Some(am) = value_map.get_mut(&asset_type) {
                    *am += amount
                } else {
                    value_map.insert(asset_type, amount);
                }
            }

            println!("{:?}", value_map);

            // println!("{:?}", result);

            //             if let Some(v) = result {
            //     let resp: GetOwnedUtxoResp = serde_json::from_value(v).c(d!())?;
            //
            //     println!("{:?}", &resp);
            // } else {
            //     println!("error !");
            //             }
        }

        Ok(())
    }
}
