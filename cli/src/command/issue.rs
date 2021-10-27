use clap::{ArgGroup, Parser};
use rand_chacha::ChaChaRng;
use rand_core::{RngCore, SeedableRng};
use ruc::{d, err::RucResult};
use zei::{
    serialization::ZeiFromToBytes,
    xfr::{
        sig::XfrSecretKey,
        structs::{AssetType, ASSET_TYPE_LENGTH},
    },
};

use crate::utils::{account_to_keypair, read_account_list};
use crate::{config::Config, utils::send_tx};
use libfn::{build_transaction, Entry, IssueEntry};

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("account_info"))]
pub struct Command {
    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short, long, group = "account_info")]
    secret_key: Option<String>,

    #[clap(short = 'a', long)]
    amount: u64,

    #[clap(short = 'A', long)]
    confidential_amount: bool,

    #[clap(short = 'i', long, group = "account_info")]
    account: Option<usize>,
}

impl Command {
    pub async fn execute(&self, config: Config) -> ruc::Result<()> {
        let keypair = if let Some(secret_key) = self.secret_key.as_ref() {
            let sk_bytes = base64::decode(secret_key).c(d!())?;
            let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;
            Some(sk.into_keypair())
        } else if let Some(account_index) = self.account {
            let mut kp = None;
            let v = read_account_list(&config).await?;
            if let Some(account) = v.get(account_index) {
                let result = account_to_keypair(account);
                if result.is_err() {
                    return Err(result.unwrap_err());
                }
                kp = result.ok();
            }
            kp
        } else {
            return Err(Box::from(d!("keypair is none")));
        };

        let mut prng = ChaChaRng::from_entropy();
        let mut asset_type = [0u8; ASSET_TYPE_LENGTH];
        prng.fill_bytes(&mut asset_type);

        println!("Asset Type is {}", base64::encode(&asset_type));

        let entry = Entry::Issue(IssueEntry {
            amount: self.amount,
            asset_type: AssetType(asset_type),
            confidential_amount: self.confidential_amount,
            keypair: keypair.unwrap(), //safe
        });

        if let Some(_e) = &self.batch {
        } else {
            let tx = build_transaction(&mut prng, vec![entry]).await?;
            log::debug!("Result tx is: {:?}", tx);

            send_tx(&tx).await?;
        }

        Ok(())
    }
}
