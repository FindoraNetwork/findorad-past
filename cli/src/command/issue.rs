use clap::Clap;
use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use ruc::*;
use zei::{
    serialization::ZeiFromToBytes,
    xfr::{
        sig::XfrSecretKey,
        structs::{AssetType, ASSET_TYPE_LENGTH},
    },
};

use crate::{
    config::Config,
    entry::{build_transaction, Entry, IssueEntry},
    utils::send_tx,
};

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short, long)]
    secret_key: String,

    #[clap(short = 'a', long)]
    amount: u64,

    #[clap(short = 'A', long)]
    confidential_amount: bool,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> Result<()> {
        let mut prng = ChaChaRng::from_entropy();

        let mut asset_type = [0u8; ASSET_TYPE_LENGTH];
        prng.fill_bytes(&mut asset_type);

        println!("Asset Type is {}", base64::encode(&asset_type));

        let sk_bytes = base64::decode(&self.secret_key).c(d!())?;
        let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;
        let keypair = sk.into_keypair();

        let entry = Entry::Issue(IssueEntry {
            amount: self.amount,
            asset_type: AssetType(asset_type),
            confidential_amount: self.confidential_amount,
            keypair,
        });

        if let Some(_e) = &self.batch {
        } else {
            let tx = build_transaction(&mut prng, vec![entry])?;
            log::debug!("Result tx is: {:?}", tx);

            send_tx(&tx).await?;
        }

        Ok(())
    }
}
