use clap::Clap;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;

use crate::{config::Config, entry::build_transaction, utils::{clean_list, read_list, send_tx}};

#[derive(Clap, Debug)]
pub struct Command {

    #[clap(short, long)]
    dump_transaction: bool,

    /// Name of batch.
    batch_name: String,
}

impl Command {
    pub async fn execute(&self, config: Config) -> Result<()> {
        let mut prng = ChaChaRng::from_entropy();

        let list = read_list(&config, &self.batch_name).await?;

        let tx = build_transaction(&mut prng, list).await?;

        if self.dump_transaction {
            let tx_json = serde_json::to_string_pretty(&tx).c(d!())?;
            println!("{}", tx_json);
        } else {
            log::debug!("Result tx is: {:?}", tx);

            send_tx(&tx).await?;

            clean_list(&config, &self.batch_name).await?;
        }

        Ok(())
    }
}
