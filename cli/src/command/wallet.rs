use clap::{ArgGroup, Parser};
use ruc::*;
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrSecretKey};

use crate::{config::Config, utils::{get_value_map,
                                    write_list,
                                    read_list,
                                    delete_one,
                                    account_to_keypair,}
};
use libfn::{AccountEntry, Entry};


#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("account"))]
pub struct Command {
    #[clap(short, long, group = "account")]
    /// Add account by mnemonic.
    add_mnemonic: Option<String>,

    #[clap(short, long, group = "account")]
    /// List account.
    list: bool,

    #[clap(short, long, group = "account")]
    /// List account.
    delete: Option<usize>,

    #[clap(short, long, group = "account")]
    /// List account.
    generate: bool,

    #[clap(short, long, group = "account")]
    /// Show account info.
    show: bool,

    wallet: Option<String>,

    #[clap(short = 'i', long)]
    /// Designated account
    account_index: Option<usize>,
}

impl Command {
    pub async fn execute(&self, config: Config) -> Result<()> {

        if self.generate {
            let entry = AccountEntry::generate_keypair()?;
            println!("{:#?}",entry);
            write_list(&config, "account",vec![Entry::Account(entry)]).await?;
            return Ok(());
        }

        if self.list {
            let v = read_list(&config,"account").await?;
            println!("{:#?}", v);
            return Ok(());
        }

        if let Some(index) = self.delete {
            let entry = delete_one(&config, "account", index).await?;
            println!("{:#?}", entry);
            return Ok(());
        }

        if let Some(phrase) = self.add_mnemonic.as_ref() {
            let entry = AccountEntry::generate_keypair_from_mnemonic(phrase)?;
            println!("{:#?}", entry);

            write_list(&config, "account",vec![Entry::Account(entry)]).await?;
            return Ok(());
        }

        if self.show {

            let mut wallets = vec![];

            if let Some(index) = &self.account_index {
                let v = read_list(&config,"account").await?;
                let entry = v.get(*index).ok_or(d!(format!("the index not exist, array len:{}",v.len())))?;
                if let Some(account) = match entry {
                    Entry::Issue(_) => {None}
                    Entry::Transfer(_) => {None}
                    Entry::Account(account) => {Some(account)}
                }{
                    let kp = account_to_keypair(account)?;
                    wallets.push(kp);
                }
            }

            if let Some(w) = &self.wallet {
                let sk_bytes = base64::decode(&w).c(d!())?;
                let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;

                let keypair = sk.into_keypair();
                wallets.push(keypair);
            }

            if wallets.len() == 0 {
                return Err(Box::from(d!("must set wallet(base64) or wallet-index(usize)")));
            }

            let value_map = get_value_map(wallets).await?;

            for (k, amount) in value_map.iter() {
                let asset_type = base64::encode(&k.zei_to_bytes());

                println!("Asset type: {}, amount: {}", asset_type, amount);
            }
        }

        Ok(())
    }
}
