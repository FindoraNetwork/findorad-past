use bech32::{ToBase32, Variant};
use bip0039::{Count, Language, Mnemonic};
use ed25519_dalek_bip32::{DerivationPath, ExtendedSecretKey};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::{serialization::ZeiFromToBytes, xfr::sig::XfrSecretKey};

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
                XfrSecretKey::zei_from_bytes(&kp.secret_key.to_bytes()[..]).map_err(|e| eg!(e))
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

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Base64KeyPair {
    pub key_pair: KeyPair,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bech32KeyPair {
    pub key_pair: KeyPair,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountEntry {
    pub mnemonic: String,
    pub base64: Base64KeyPair,
    pub bech32: Bech32KeyPair,
}

pub struct BipPath {
    pub coin: u32,
    pub account: u32,
    pub change: u32,
    pub address: u32,
}

impl BipPath {
    pub fn new_fra() -> Self {
        Self {
            coin: 917,
            account: 0,
            change: 0,
            address: 0,
        }
    }
}

impl AccountEntry {
    pub fn generate_keypair() -> Result<AccountEntry> {
        let mnemonic = pnk!(generate_mnemonic_custom(24, "en"));
        AccountEntry::generate_keypair_from_mnemonic(&*mnemonic)
    }

    pub fn generate_keypair_from_mnemonic(mnemonic: &str) -> Result<AccountEntry> {
        let kp = restore_keypair_from_mnemonic!(mnemonic.clone(), "en", BipPath::new_fra(), bip44)
            .c(d!())?;
        let base64_pub_key = base64::encode(&kp.get_pk().zei_to_bytes());
        let base64_sec_key = base64::encode(&kp.get_sk().zei_to_bytes());

        let bech32_pub_key = bech32::encode(
            "fra",
            &kp.get_pk().zei_to_bytes().to_base32(),
            Variant::Bech32,
        )
        .c(d!())?;
        let bech32_sec_key = bech32::encode(
            "fra",
            &kp.get_sk().zei_to_bytes().to_base32(),
            Variant::Bech32,
        )
        .c(d!())?;

        Ok(Self {
            mnemonic: mnemonic.to_string(),
            base64: Base64KeyPair {
                key_pair: KeyPair {
                    public_key: base64_pub_key,
                    secret_key: base64_sec_key,
                },
            },
            bech32: Bech32KeyPair {
                key_pair: KeyPair {
                    public_key: bech32_pub_key,
                    secret_key: bech32_sec_key,
                },
            },
        })
    }
}
