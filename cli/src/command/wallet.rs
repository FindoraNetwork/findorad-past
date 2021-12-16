use clap::{ArgEnum, Parser};
// use ruc::d;
// use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrSecretKey};
//
// use crate::{
//     config::Config,
//     utils::{
//         account_to_keypair, delete_account_one, get_value_map, read_account_list,
//         write_account_list,
//     },
// };
// use libfn::AccountEntry;

use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    /// Show a list of wallet addresses or specific one for detail information
    Show(Show),
    /// Create a new wallet or create a new wallet from Mnemonic phrase
    Create(Create),
    /// Delete a wallet
    Delete(Delete),
}

#[derive(Parser, Debug)]
struct Show {
    /// Wallet address to show the wallet information of the specific one
    #[clap(short, long, forbid_empty_values = true)]
    address: Option<String>,
}

#[derive(Parser, Debug)]
struct Create {
    /// Specific a wallet type to create
    #[clap(arg_enum, short, long, default_value = "findora")]
    wallet_typ: KeyType,
    /// Specific to create a new wallet from Mnemonic phrase
    #[clap(short, long, forbid_empty_values = true)]
    mnemonic: Option<String>,
}

#[derive(ArgEnum, Debug, Clone)]
enum KeyType {
    /// Generate a random Findora public and private key pair
    FRA,
    /// Generate an Ethereum public and private key pair from??? and memo too???
    ETH,
}

#[derive(Parser, Debug)]
struct Delete {
    /// Wallet address to do the deletion
    #[clap(forbid_empty_values = true)]
    address: String,
}

impl Command {
    pub async fn execute(&self, _config: Config) -> ruc::Result<()> {
        match &self.subcmd {
            SubCommand::Show(_show) => {}
            SubCommand::Create(_create) => {}
            SubCommand::Delete(_delete) => {}
        }
        // use ruc::*;

        // if self.generate {
        //     let entry = AccountEntry::generate_keypair()?;
        //     println!("{:#?}", entry);
        //     write_account_list(&config, vec![entry], false).await?;
        //     return Ok(());
        // }

        // if self.list {
        //     let v = read_account_list(&config).await?;
        //     println!("{:#?}", v);
        //     return Ok(());
        // }

        // if let Some(index) = self.delete {
        //     let entry = delete_account_one(&config, index).await?;
        //     println!("{:#?}", entry);
        //     return Ok(());
        // }

        // if let Some(phrase) = self.add_mnemonic.as_ref() {
        //     let entry = AccountEntry::generate_keypair_from_mnemonic(phrase)?;
        //     println!("{:#?}", entry);

        //     write_account_list(&config, vec![entry], false).await?;
        //     return Ok(());
        // }

        // if self.show {
        //     let mut wallets = vec![];

        //     if let Some(index) = &self.account_index {
        //         let v = read_account_list(&config).await?;
        //         let account = v
        //             .get(*index)
        //             .ok_or(d!(format!("the index not exist, array len:{}", v.len())))?;
        //         let kp = account_to_keypair(account)?;
        //         wallets.push(kp);
        //     }

        //     if let Some(w) = &self.wallet {
        //         let sk_bytes = base64::decode(&w).c(d!())?;
        //         let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;

        //         let keypair = sk.into_keypair();
        //         wallets.push(keypair);
        //     }

        //     if wallets.len() == 0 {
        //         return Err(Box::from(d!(
        //             "must set wallet(base64) or wallet-index(usize)"
        //         )));
        //     }

        //     let value_map = get_value_map(wallets).await?;

        //     for (k, amount) in value_map.iter() {
        //         let asset_type = base64::encode(&k.zei_to_bytes());

        //         println!("Asset type: {}, amount: {}", asset_type, amount);
        //     }
        // }

        Ok(())
    }
}
