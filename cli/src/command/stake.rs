use clap::{ArgGroup, Parser};
use libfindora::staking::TendermintAddress;
use libfn::{build_transaction, DelegationEntry, Entry, TdPubkeyType, UnDelegationEntry};
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;
use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::path::Path;
use zei::serialization::ZeiFromToBytes;
use zei::xfr::sig::{XfrKeyPair, XfrSecretKey};

use crate::config::Config;
use crate::utils::{
    account_to_keypair, get_delegation_info, query_validators, read_account_list, send_tx,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TDKey {
    pub address: String,
    pub pub_key: BTreeMap<String, String>,
}

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("operation"))]
#[clap(group = ArgGroup::new("account_info"))]
pub struct Command {
    #[clap(short = 'L', long, group = "operation")]
    /// List all available validator node's info.
    list_validators: bool,

    #[clap(short = 'S', long, group = "operation")]
    /// Get findorad rpc address.
    get_stake_info: bool,

    #[clap(short, long)]
    /// Special a batch name.
    batch: Option<String>,

    #[clap(short = 'f', long, group = "account_info")]
    /// From secret key.
    from_secret_key: Option<String>,

    #[clap(short = 'i', long, group = "account_info")]
    account: Option<usize>,

    #[clap(short, long)]
    validator_address: Option<String>,

    #[clap(short = 'D', long, group = "operation")]
    /// delegation
    delegation: bool,

    #[clap(short = 'd', long, group = "operation")]
    /// delegation
    delegation_self: Option<String>,

    #[clap(short = 'U', long, group = "operation")]
    /// un-delegation
    undelegation: bool,

    #[clap(short = 'a', long)]
    amount: Option<u64>,
}

impl Command {
    async fn get_kp(&self, config: &Config) -> Result<XfrKeyPair> {
        let kp = if let Some(sk) = &self.from_secret_key {
            let sk_bytes = base64::decode(sk).c(d!())?;
            let sk = XfrSecretKey::zei_from_bytes(&sk_bytes).c(d!())?;
            sk.into_keypair()
        } else if let Some(index) = self.account {
            let v = read_account_list(config).await.c(d!())?;
            if let Some(account) = v.get(index) {
                let local_kp = account_to_keypair(account).c(d!())?;
                local_kp
            } else {
                return Err(eg!("index cannot be matched"));
            }
        } else {
            return Err(eg!("input account index or secret key"));
        };
        Ok(kp)
    }

    async fn get_amount(&self) -> Result<u64> {
        return if let Some(a) = self.amount {
            Ok(a)
        } else {
            Err(eg!("must input amount"))
        };
    }

    async fn send_tx(&self, entries: Vec<Entry>) -> Result<()> {
        let mut prng = ChaChaRng::from_entropy();

        let tx = build_transaction(&mut prng, entries).await?;

        log::debug!("Result tx is: {:?}", tx);

        send_tx(&tx).await?;

        Ok(())
    }

    pub async fn execute(&self, config: Config) -> Result<()> {
        if self.list_validators {
            query_validators().await?;
            return Ok(());
        }

        if self.get_stake_info {
            let mut wallets = vec![];

            if let Some(sk) = &self.from_secret_key {
                let sk_bytes = base64::decode(sk).c(d!())?;
                let sk = XfrSecretKey::zei_from_bytes(&sk_bytes).c(d!())?;
                wallets.push(sk.into_keypair());
            }

            if let Some(index) = self.account {
                let v = read_account_list(&config).await.c(d!())?;
                if let Some(account) = v.get(index) {
                    let kp = account_to_keypair(account).c(d!())?;
                    wallets.push(kp);
                }
            }

            get_delegation_info(wallets).await?;
            return Ok(());
        }

        if self.delegation || self.undelegation {
            let kp = self.get_kp(&config).await?;

            let amount = self.get_amount().await?;

            let td_addr = if let Some(address) = &self.validator_address {
                let addr_bytes = hex::decode(address).c(d!())?;
                println!("len:{:?}", addr_bytes.len());
                let address = TendermintAddress::new_from_vec(&addr_bytes.to_vec());
                address
            } else {
                return Err(eg!("must input validator address"));
            };

            let entry = if self.delegation {
                Entry::Delegation(DelegationEntry {
                    keypair: kp,
                    amount,
                    validator_address: td_addr,
                    validator_ty_pubkey: None,
                })
            } else {
                Entry::UnDelegation(UnDelegationEntry {
                    keypair: kp,
                    amount,
                    validator_address: td_addr,
                })
            };

            self.send_tx(vec![entry]).await?;
        }

        if let Some(key_path) = &self.delegation_self {
            let kp = self.get_kp(&config).await?;

            let amount = self.get_amount().await?;

            let path = key_path.clone() + "/config/priv_validator_key.json";
            log::debug!("path:{}", path);

            let path = Path::new(&path);
            let key_str = read_to_string(path.clone()).c(d!())?;
            let key: TDKey = serde_json::from_str(&key_str).c(d!())?;

            // must exist,else panic!
            let addr_bytes = hex::decode(key.address).c(d!())?;
            let td_addr = TendermintAddress::new_from_vec(&addr_bytes);

            let ty = key
                .pub_key
                .get("type")
                .ok_or(eg!("not exist type"))?
                .clone();
            let value_base64 = key.pub_key.get("value").ok_or(eg!("not exist value"))?;
            let value = base64::decode(value_base64).c(d!())?;

            let v: Vec<&str> = ty.split("/").collect();
            let ty_enum = TdPubkeyType::from_str(v[1])?;
            let entry = Entry::Delegation(DelegationEntry {
                keypair: kp,
                amount,
                validator_address: td_addr,
                validator_ty_pubkey: Some((ty_enum, value)),
            });

            self.send_tx(vec![entry]).await?;
        }

        Ok(())
    }
}
