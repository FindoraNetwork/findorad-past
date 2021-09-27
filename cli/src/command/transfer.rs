use clap::Clap;
use ruc::*;

use crate::config::Config;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short, long)]
    secret_key: String,

    #[clap(short, long)]
    amount: u64,

    #[clap(short, long)]
    asset_type: String,

    #[clap(short, long)]
    target: String,
}

impl Command {
    pub fn execute(&self, config: Config) -> Result<()> {

//         if let Some(batch) = self.batch.clone() {
        //     let mut path = config.node.home;
        //     path.push("transfer");
        //
        //     if !batch.eq("execute") {
        //         let tbe = self.check().c(d!())?;
        //
        //         path.push(batch);
        //
        //         save_transfer_to_batch(&path, tbe).c(d!())?;
        //
        //     } else {
        //         let dir = std::fs::read_dir(path.as_path()).c(d!())?;
        //         let mut batch_map = HashMap::new();
        //         for entry in dir {
        //             let e = entry.c(d!())?;
        //             let file_path = e.path();
        //             let m = read_transfer_from_batch(&file_path).c(d!())?;
        //             batch_map.extend(m);
        //         }
        //
        //         let tx = transfer_tx(batch_map).c(d!())?;
        //         send_tx(&tx).c(d!())?;
        //     }
        //
        //     return Ok(())
        // }
        //
        // let tbe = self.check().c(d!())?;
        // let mut map = HashMap::new();
        // map.insert(tbe.from.pub_key,vec![tbe]);
        //
        // let tx = transfer_tx(map).c(d!())?;
        // send_tx(&tx).c(d!())?;
//
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


