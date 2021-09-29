use std::convert::TryInto;

use clap::Clap;
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
    utils::{send_tx, write_list},
};

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,
    #[clap(short = 'f', long)]
    /// From secret key.
    from_secret_key: String,

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
}

impl Command {
    pub async fn execute(&self, config: Config) -> Result<()> {
        let mut prng = ChaChaRng::from_entropy();

        let asset_type_bytes = base64::decode(&self.asset_type).c(d!())?;
        let asset_type_array: [u8; ASSET_TYPE_LENGTH] = asset_type_bytes
            .try_into()
            .map_err(|e| eg!(format!("{:?}", e)))?;
        let asset_type = AssetType(asset_type_array);

        let from_sk_bytes = base64::decode(&self.from_secret_key).c(d!())?;
        let from_sk = XfrSecretKey::zei_from_bytes(&from_sk_bytes)?;
        let from = from_sk.into_keypair();

        let to_pk_bytes = base64::decode(&self.to_public_key).c(d!())?;
        let to = XfrPublicKey::zei_from_bytes(&to_pk_bytes)?;

        let entry = Entry::Transfer(TransferEntry {
            confidential_amount: self.confidential_amount,
            confidential_asset: self.confidential_asset,
            amount: self.amount,
            asset_type,
            from,
            to,
        });

        if let Some(b) = &self.batch {
            write_list(&config, b, vec![entry]).await?;
        } else {
            let tx = build_transaction(&mut prng, vec![entry]).await?;
            log::debug!("Result tx is: {:?}", tx);

            send_tx(&tx).await?;
        }

        Ok(())
    }

    //  fn check(&self) -> Result<TransferBatchEntry> {
    // let secret_key = self.secret_key.clone().ok_or(d!("secret key must set"))?;
    // let from = secret_key_to_keypair(secret_key).c(d!())?;
    // let amount = self.amount.clone().ok_or(d!("amount must set"))?;
    // let target = self.target.clone().ok_or(d!("target must set"))?;
    // let to = public_key_from_base64(target).c(d!())?;
    //
    // let asset_type = self.asset_type.clone().unwrap_or({
    //     let mut rng = thread_rng();
    //     let chars: String = iter::repeat(())
    //         .map(|()| rng.sample(Alphanumeric))
    //         .map(char::from)
    //         .take(32)
    //         .collect();
    //     chars
    // }).as_bytes().to_vec();
    //
    // if asset_type.len() > 32 {
    //     return Err(Box::from(d!("asset type must be less than or equal to 32 bits")));
    // }
    //
    // let mut at = [0_u8;32];
    // for (index,n) in asset_type.iter().enumerate() {
    //     at[index] = *n;
    // }
    //
    // let tbe = TransferBatchEntry{
    //     from,
    //     to,
    //     amount,
    //     asset_type: XfrAssetType::NonConfidential(AssetType{ 0:at }),
    // };
    //
    // Ok(tbe)
    //  }
}
