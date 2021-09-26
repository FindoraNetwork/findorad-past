use clap::Clap;
use ruc::*;

use crate::config::Config;
use crate::utils::issue_asset;

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
    asset_type: u8,
}

impl Command {
    pub fn execute(&self, _config: Config) -> Result<()> {

        let mut batch_file = None;

        if let Some(batch) = self.batch.clone() {
            let path = "~/__tx_issue_batch/";
            let file = path.to_string() + &*batch;
            std::fs::create_dir_all(path).c(d!())?;
            batch_file = Some(file);
        }

        issue_asset(self.secret_key.clone(),self.amount,self.asset_type,batch_file).c(d!())?;

        Ok(())
    }
}
