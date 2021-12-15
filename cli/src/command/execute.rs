use clap::Parser;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
// use ruc::*;
use crate::config::Config;
// use crate::{
//     config::Config,
//     utils::{clean_list, read_list, send_tx},
// };
// use libfn::build_transaction;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(short, long)]
    dump_transaction: bool,

    /// Name of batch.
    batch_name: String,
}

impl Command {
    pub async fn execute(&self, config: Config) -> ruc::Result<()> {
        // let mut prng = ChaChaRng::from_entropy();

        // let list = read_list(&config, &self.batch_name).await?;

        // let tx = build_transaction(&mut prng, list).await?;

        // if self.dump_transaction {
        //     println!("{:#?}", tx);
        // } else {
        //     log::debug!("Result tx is: {:?}", tx);

        //     send_tx(&tx).await?;

        //     clean_list(&config, &self.batch_name).await?;
        // }

        Ok(())
    }
}
