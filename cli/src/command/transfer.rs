use std::convert::TryInto;

use clap::{ArgGroup, Clap};
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;
use zei::{
    serialization::ZeiFromToBytes,
    xfr::{
        sig::{XfrPublicKey, XfrSecretKey},
        structs::{AssetType, ASSET_TYPE_LENGTH},
    },
};

use crate::{
    config::Config,
    entry::{build_transaction, Entry, TransferEntry},
    utils::send_tx,
};
use crate::entry::wallet::AccountEntry;

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("transfer"))]
pub struct Command {
    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short = 'f', long, group = "transfer")]
    /// From secret key.
    from_secret_key: Option<String>,

    #[clap(short = 'a', long)]
    amount: u64,

    #[clap(short = 't', long)]
    asset_type: String,

    #[clap(short = 'd', long)]
    to_public_key: String,

    #[clap(short = 'A', long)]
    confidential_amount: bool,

    #[clap(short = 'T', long)]
    confidential_asset: bool,

    #[clap(short, long, group = "transfer")]
    account_index: Option<usize>,

}

impl Command {
    pub async fn execute(&self, config: Config) -> Result<()> {

        let mut from = None;

        if let Some(from_secret_key) = self.from_secret_key.as_ref() {
            let from_sk_bytes = base64::decode(from_secret_key).c(d!())?;
            let from_sk = XfrSecretKey::zei_from_bytes(&from_sk_bytes)?;
            from = Some(from_sk.into_keypair());
        }

        if let Some(account_index) = self.account_index {
            from = Some(AccountEntry::from_index_to_keypair(account_index, &config)?);
        }

        if from.is_none() {
            return Err(Box::from(d!("keypair is none")));
        }

        let mut prng = ChaChaRng::from_entropy();

        let asset_type_bytes = base64::decode(&self.asset_type).c(d!())?;
        let asset_type_array: [u8; ASSET_TYPE_LENGTH] = asset_type_bytes
            .try_into()
            .map_err(|e| eg!(format!("{:?}", e)))?;
        let asset_type = AssetType(asset_type_array);

        let to_pk_bytes = base64::decode(&self.to_public_key).c(d!())?;
        let to = XfrPublicKey::zei_from_bytes(&to_pk_bytes)?;

        if let Some(_b) = &self.batch {
        } else {
            let entry = Entry::Transfer(TransferEntry {
                confidential_amount: self.confidential_amount,
                confidential_asset: self.confidential_asset,
                amount: self.amount,
                asset_type,
                from: from.unwrap(),//safe
                to,
            });

            let tx = build_transaction(&mut prng, vec![entry]).await?;
            log::debug!("Result tx is: {:?}", tx);

            send_tx(&tx).await?;
        }

        Ok(())
    }
}
