use clap::{ArgGroup, Clap};
use ruc::*;
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrSecretKey};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use bip0039::{Count, Language, Mnemonic};

use crate::config::Config;
use crate::entry::{AccountEntry, BipPath};

macro_rules! restore_keypair_from_mnemonic {
    ($phrase: expr, $l: expr, $p: expr, $bip: tt) => {
        check_lang($l)
            .c(d!())
            .and_then(|l| Mnemonic::from_phrase_in(l, $phrase).map_err(|e| eg!(e)))
            .map(|m| m.to_seed(""))
            .and_then(|seed| {
                DerivationPath::$bip($p.coin, $p.account, $p.change, $p.address)
                    .map_err(|e| eg!(e))
                    .map(|dp| (seed, dp))
            })
            .and_then(|(seed, dp)| {
                ExtendedSecretKey::from_seed(&seed)
                    .map_err(|e| eg!(e))?
                    .derive(&dp)
                    .map_err(|e| eg!(e))
            })
            .and_then(|kp| {
                XfrSecretKey::zei_from_bytes(&kp.secret_key.to_bytes()[..])
                    .map_err(|e| eg!(e))
            })
            .map(|sk| sk.into_keypair())
    };
}

fn generate_mnemonic_custom(wordslen: u8, lang: &str) -> Result<String> {
    let w = match wordslen {
        12 => Count::Words12,
        15 => Count::Words15,
        18 => Count::Words18,
        21 => Count::Words21,
        24 => Count::Words24,
        _ => {
            return Err(eg!(
                "Invalid words length, only 12/15/18/21/24 can be accepted."
            ));
        }
    };

    let l = check_lang(lang).c(d!())?;

    Ok(Mnemonic::generate_in(l, w).into_phrase())
}

pub fn check_lang(lang: &str) -> Result<Language> {
    match lang {
        "en" => Ok(Language::English),
        "zh" => Ok(Language::SimplifiedChinese),
        "zh_traditional" => Ok(Language::TraditionalChinese),
        "fr" => Ok(Language::French),
        "it" => Ok(Language::Italian),
        "ko" => Ok(Language::Korean),
        "sp" => Ok(Language::Spanish),
        "jp" => Ok(Language::Japanese),
        _ => Err(eg!("Unsupported language")),
    }
}

#[derive(Clap, Debug)]
#[clap(group = ArgGroup::new("account"))]
pub struct Command {
    #[clap(short, long, group = "account")]
    /// Add account by mnemonic.
    add_mnemonic: Option<String>,

    #[clap(short, long, group = "account")]
    /// List account.
    list_account: bool,

    #[clap(short, long, group = "account")]
    /// List account.
    delete_account: Option<usize>,

    #[clap(short, long, group = "account")]
    /// List account.
    generate: bool,

}

impl Command {
    pub async fn execute(&self, config: Config) -> Result<()> {

        if self.generate {
            let mnemonic = pnk!(generate_mnemonic_custom(24, "en"));
            let keypair = restore_keypair_from_mnemonic!(mnemonic.clone(), "en", BipPath::new_fra(), bip44)
                .c(d!())?;

            let ae = AccountEntry::form_keypair(keypair, mnemonic)?;

            println!("{:#?}",ae);

            AccountEntry::save(&mut vec![ae], &config, false)?;
            return Ok(());
        }

        if self.list_account {
            let v = AccountEntry::read(&config)?;


            println!("{:#?}",v);
            return Ok(());
        }

        if let Some(index) = self.delete_account {

            let ae = AccountEntry::delete(index, &config)?;

            println!("{:#?}",ae);
            return Ok(())
        }

        if let Some(phrase) = self.add_mnemonic.as_ref(){

            let keypair = restore_keypair_from_mnemonic!(phrase, "en", BipPath::new_fra(), bip44)
                .c(d!())?;

            let ae = AccountEntry::form_keypair(keypair, phrase.clone())?;

            println!("{:#?}",ae);

            AccountEntry::save(&mut vec![ae], &config, false)?;
            return Ok(());
        }


        Ok(())
    }
}
