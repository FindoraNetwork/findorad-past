use clap::{ArgGroup, Clap};
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;
use zei::{
    serialization::ZeiFromToBytes,
    xfr::sig::{XfrKeyPair, XfrSecretKey},
};

use crate::{config::Config, utils::get_value_map};

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

            let value_map = get_value_map(wallets).await?;

            for (k, amount) in value_map.iter() {
                let asset_type = base64::encode(&k.zei_to_bytes());

                println!("Asset type: {}, amount: {}", asset_type, amount);
            }
        }

        Ok(())
    }
}
