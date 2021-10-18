use crate::config::Config;
use bech32::{ToBase32, Variant};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::serialization::ZeiFromToBytes;
use zei::xfr::sig::XfrKeyPair;
use zei::xfr::sig::XfrSecretKey;

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
    pub fn form_keypair(kp: XfrKeyPair, phrase: String) -> Result<Self> {
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
            mnemonic: phrase,
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

    pub fn to_keypair(&self) -> Result<XfrKeyPair> {
        let sk_bytes = base64::decode(&self.base64.key_pair.secret_key).c(d!())?;
        let sk = XfrSecretKey::zei_from_bytes(&sk_bytes)?;

        let keypair = sk.into_keypair();
        Ok(keypair)
    }

    pub fn from_index_to_keypair(index: usize, config: &Config) -> Result<XfrKeyPair> {
        let aes = Self::read(&config)?;
        let ae = aes.get(index);

        if ae.is_none() {
            return Err(Box::from(d!(
                "There is no corresponding account for this index"
            )));
        }

        // safe
        let ae = ae.unwrap();
        let kp = ae.to_keypair()?;

        Ok(kp)
    }

    pub fn save(aes: &mut Vec<AccountEntry>, config: &Config, is_cover: bool) -> Result<()> {
        let mut path = config.node.home.clone();
        path.push("_account");

        let bytes = if !is_cover {
            let mut vec = Self::read(config).c(d!())?;
            vec.append(aes);
            serde_json::to_vec(&vec).c(d!())?
        } else {
            serde_json::to_vec(aes).c(d!())?
        };

        std::fs::write(path.as_path(), bytes).c(d!())?;

        Ok(())
    }

    pub fn read(config: &Config) -> Result<Vec<AccountEntry>> {
        let mut path = config.node.home.clone();
        path.push("_account");

        if !path.as_path().exists() {
            std::fs::File::create(path.as_path()).c(d!())?;
            let vec: Vec<AccountEntry> = Vec::new();
            return Ok(vec);
        }

        let data = std::fs::read(path).c(d!())?;
        let vec = serde_json::from_slice::<Vec<AccountEntry>>(data.as_slice()).c(d!())?;

        Ok(vec)
    }

    pub fn delete(index: usize, config: &Config) -> Result<AccountEntry> {
        let mut vec = Self::read(config).c(d!())?;
        let ae = vec.remove(index);

        Self::save(&mut vec, config, true).c(d!())?;
        Ok(ae)
    }
}
