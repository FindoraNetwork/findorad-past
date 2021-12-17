// use std::convert::TryInto;

use clap::{Parser, ValueHint};
// use rand_chacha::ChaChaRng;
// use rand_core::SeedableRng;
// use ruc::{d, eg};
// use zei::{
//     serialization::ZeiFromToBytes,
//     xfr::{
//         // sig::{XfrPublicKey, XfrSecretKey},
//         sig::XfrSecretKey,
//         structs::{AssetType, ASSET_TYPE_LENGTH},
//     },
// };
use crate::config::Config;
// use crate::{
//     config::Config,
//     utils::{account_to_keypair, read_account_list, send_tx},
// };
// use libfn::{build_transaction, Entry, TransferEntry};

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Send tokens from the wallet to a specific address directly
    Send(Send),
    /// Save this sending request for further bach sending them together
    Save(Save),
    /// Batch sending the saved requests
    Batch(Batch),
    /// Show a list of saved batch process names or specific one for detail information
    Show(Show),
}

#[derive(Parser, Debug)]
struct Send {
    /// To specific a file path to the Findora wallet which contains base64-formatted XfrPrivateKey
    /// This option cannot be used with --from-secret-string
    #[clap(group = "from")]
    #[clap(short = 'f', long, value_name = "PATH", forbid_empty_values = true, value_hint = ValueHint::FilePath)]
    from_secret_file: Option<std::path::PathBuf>,
    /// To specific a plain-text input as the Findora wallet which is a base64-formatted XfrPrivateKey
    /// This option cannot be used with --from-secret-file
    #[clap(group = "from")]
    #[clap(short = 's', long, value_name = "STRING", forbid_empty_values = true)]
    from_secret_string: Option<String>,
    /// Amount of FRA tokens to send
    #[clap(short, long, required = true, forbid_empty_values = true)]
    amount: u64,
    /// Address to send which is a base64-formated XfrPublicKey
    #[clap(forbid_empty_values = true)]
    to_address: String,
    /// Make the amount confidential in the transaction
    #[clap(short = 'A', long)]
    confidential_amount: bool,
    /// Make the asset code confidential in the transaction
    #[clap(short = 'T', long)]
    confidential_asset: bool,
}

#[derive(Parser, Debug)]
struct Save {
    /// Name of the batch process for identifying in the batch command
    /// Save with the same batch name will appending the new request
    #[clap(short, long, required = true, forbid_empty_values = true)]
    batch_name: String,
    #[clap(flatten)]
    request: Send,
}

#[derive(Parser, Debug)]
struct Batch {
    /// Name of the batch process will be executing
    #[clap(forbid_empty_values = true)]
    batch_name: String,
}

#[derive(Parser, Debug)]
struct Show {
    /// Name of the batch process to show the request information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    batch_name: Option<String>,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        // use ruc::*;

        // let from = if let Some(from_secret_key) = self.from_secret_key.as_ref() {
        //     let from_sk_bytes = base64::decode(from_secret_key).c(d!())?;
        //     let from_sk = XfrSecretKey::zei_from_bytes(&from_sk_bytes)?;
        //     Some(from_sk.into_keypair())
        // } else if let Some(account_index) = self.account_index {
        //     let mut kp = None;
        //     let v = read_account_list(&config).await?;
        //     if let Some(account) = v.get(account_index) {
        //         let result = account_to_keypair(account);
        //         if result.is_err() {
        //             return Err(result.unwrap_err());
        //         }
        //         kp = result.ok();
        //     }
        //     kp
        // } else {
        //     return Err(eg!("keypair is none"));
        // };

        // let mut prng = ChaChaRng::from_entropy();

        // let asset_type_bytes = base64::decode(&self.asset_type).c(d!())?;
        // let asset_type_array: [u8; ASSET_TYPE_LENGTH] = asset_type_bytes
        //     .try_into()
        //     .map_err(|e| eg!(format!("{:?}", e)))?;
        // let asset_type = AssetType(asset_type_array);

        // let to_pk_bytes = base64::decode(&self.to_public_key).c(d!())?;
        // let to = XfrPublicKey::zei_from_bytes(&to_pk_bytes)?;

        // if let Some(_b) = &self.batch {
        // } else {
        //     let entry = Entry::Transfer(TransferEntry {
        //         confidential_amount: self.confidential_amount,
        //         confidential_asset: self.confidential_asset,
        //         amount: self.amount,
        //         asset_type,
        //         from: from.unwrap(), //safe
        //         to,
        //     });

        //     let tx = build_transaction(&mut prng, vec![entry]).await?;
        //     log::debug!("Result tx is: {:?}", tx);

        //     send_tx(&tx).await?;
        // }
        //
        // ++++++++++++++++++++++++++++
        // the original execute.rs code
        // ++++++++++++++++++++++++++++
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
