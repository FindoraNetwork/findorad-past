use clap::Clap;
use ruc::*;

use crate::config::Config;

#[derive(Clap, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short, long)]
    secret_key: Option<String>,

    #[clap(short = 'a', long)]
    amount: u64,

    #[clap(short = 't', long)]
    asset_type: u8,

    #[clap(short = 'A', long)]
    confidential_amount: bool,
}

impl Command {
    pub fn execute(&self, config: Config) -> Result<()> {

   //      if let Some(batch) = self.batch.clone() {
        //
        //     let mut path = config.node.home;
        //     path.push("issue");
        //
        //     if !batch.eq("execute") {
        //
        //         let ibe = self.check().c(d!())?;
        //
        //         path.push(batch);
        //
        //         save_issue_to_batch(&path, ibe).c(d!())?;
        //
        //     } else {
        //         let dir = std::fs::read_dir(path.as_path()).c(d!())?;
        //         let mut batch_vec = vec![];
        //         for entry in dir {
        //             let e = entry.c(d!())?;
        //             let file_path = e.path();
        //             let mut v = read_issue_from_batch(&file_path).c(d!())?;
        //             batch_vec.append(&mut v);
        //         }
        //
        //         let tx = issue_tx(batch_vec).c(d!())?;
        //         send_tx(&tx).c(d!())?;
        //
        //     }
        //     return Ok(());
        // }
        //
        // let ibe = self.check().c(d!())?;
        //
        // let tx = issue_tx(vec![ibe]).c(d!())?;
        // send_tx(&tx).c(d!())?;
//
        Ok(())
    }

//     fn check(&self) -> Result<IssueBatchEntry> {
        // let secret_key = self.secret_key.clone().ok_or(d!("secret key must set"))?;
        // let keypair = secret_key_to_keypair(secret_key)?;
        // let amount = self.amount.clone().ok_or(d!("amount must set"))?;
        //
        // let asset_type = self.asset_type;
        //
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
        // let ibe = IssueBatchEntry{
        //     keypair,
        //     amount,
        //     asset_type: XfrAssetType::NonConfidential(AssetType{ 0:at }),
        // };
        // Ok(ibe)
//     }
}
