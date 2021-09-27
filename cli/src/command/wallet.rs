use clap::{Clap, ArgGroup};
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrKeyPair};

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
}

impl Command {
    pub fn execute(&self, _config: Config) -> Result<()> {
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
        Ok(())
    }
}
