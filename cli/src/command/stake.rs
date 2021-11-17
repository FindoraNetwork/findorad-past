use clap::{ArgGroup, Parser};
use libfindora::staking::TendermintAddress;
use libfn::{build_transaction, DelegationEntry, Entry, UnDelegationEntry};
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use ruc::*;
use zei::serialization::ZeiFromToBytes;
use zei::xfr::sig::XfrSecretKey;

use crate::config::Config;
use crate::utils::{
    account_to_keypair, get_delegation_info, query_validators, read_account_list, send_tx,
};

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
    validator: Option<String>,

    #[clap(short = 'D', long, group = "operation")]
    /// delegation
    delegation: bool,

    #[clap(short = 'U', long, group = "operation")]
    /// un-delegation
    undelegation: bool,

    #[clap(long)]
    amount: u64,
}

impl Command {
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
            let kp = if let Some(sk) = &self.from_secret_key {
                let sk_bytes = base64::decode(sk).c(d!())?;
                let sk = XfrSecretKey::zei_from_bytes(&sk_bytes).c(d!())?;
                sk.into_keypair()
            } else if let Some(index) = self.account {
                let v = read_account_list(&config).await.c(d!())?;
                if let Some(account) = v.get(index) {
                    let local_kp = account_to_keypair(account).c(d!())?;
                    local_kp
                } else {
                    return Err(eg!("index cannot be matched"));
                }
            } else {
                return Err(eg!("input account index or secret key"));
            };

            let address = if let Some(address) = &self.validator {
                let addr_bytes = base64::decode(address).c(d!())?;
                let address = TendermintAddress::new_from_vec(&addr_bytes);
                address
            } else {
                return Err(eg!("must input validator address"));
            };

            let entry = if self.delegation {
                Entry::Delegation(DelegationEntry {
                    keypair: kp,
                    amount: self.amount,
                    validator_address: address,
                })
            } else {
                Entry::UnDelegation(UnDelegationEntry {
                    keypair: kp,
                    amount: self.amount,
                    validator_address: address,
                })
            };

            let mut prng = ChaChaRng::from_entropy();

            let tx = build_transaction(&mut prng, vec![entry]).await?;

            log::debug!("Result tx is: {:?}", tx);

            send_tx(&tx).await?;
        }

        Ok(())
    }
}
