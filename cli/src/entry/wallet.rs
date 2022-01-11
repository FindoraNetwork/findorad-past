use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use libfn::types;
use serde::{Deserialize, Serialize};

const DEFAULT_WALLET_FILE: &str = "wallets.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Wallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub mnemonic: String,
    pub address: String,
    pub public: String,
    pub secret: String,
}

pub struct WalletInfo {
    pub name: Option<String>,
    pub eth_compatible_address: String,
    pub fra_address: String,
}

pub struct ReadBuilder<'a> {
    wallets: &'a Wallets,
    address: Option<&'a str>,
    secret: Option<&'a str>,
}

impl<'a> ReadBuilder<'a> {
    pub fn new(wallets: &Wallets) -> ReadBuilder {
        ReadBuilder {
            wallets,
            address: None,
            secret: None,
        }
    }

    pub fn from_address(&mut self, addr: &'a str) -> &ReadBuilder {
        self.address = Some(addr);
        self
    }

    pub fn from_secret(&mut self, secret: &'a str) -> &ReadBuilder {
        self.secret = Some(secret);
        self
    }

    pub fn build(&self) -> Result<Wallet> {
        let result = if let Some(address) = self.address {
            let addr = detect_address(address)?;
            self.wallets.wallets.iter().find(|w| w.address == addr)
        } else if let Some(secret) = self.secret {
            self.wallets.wallets.iter().find(|w| w.secret == secret)
        } else {
            None
        };

        match result {
            Some(w) => Ok(w.clone()),
            None => bail!("read connot find"),
        }
    }
}

pub struct Wallets {
    wallets: Vec<Wallet>,
    f_path: PathBuf,
}

impl Wallet {
    pub fn to_fra_address(&self) -> Result<String> {
        types::Address::from_base64(&self.address)
            .with_context(|| format!("to_fra_address::from_base64 failed: {}", self.address))?
            .to_bech32()
            .with_context(|| format!("to_fra_address::to_bech32 failed: {}", self.address))
    }

    pub fn to_eth_address(&self) -> Result<String> {
        types::Address::from_base64(&self.address)
            .with_context(|| format!("to_eth_address::from_base64 failed: {}", self.address))?
            .to_eth()
            .with_context(|| format!("to_eth_address::to_eth failed: {}", self.address))
    }
}

impl Wallets {
    pub fn new(home: &Path) -> Result<Wallets> {
        let f_path = home.join(DEFAULT_WALLET_FILE);
        if !f_path.exists() {
            fs::write(f_path.clone(), "[]")?;
        }

        let data = fs::read(f_path.clone())
            .with_context(|| format!("new read json file failed: {:?}", f_path))?;
        let wallets: Vec<Wallet> = serde_json::from_slice(&data)
            .with_context(|| format!("new deserialize json failed: {:?}", f_path))?;

        Ok(Wallets { wallets, f_path })
    }

    pub fn create(&mut self, wallet: &Wallet) -> Result<()> {
        self.wallets.push(wallet.clone());
        self.save()
            .with_context(|| format!("create on save failed: {:?}", wallet))?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_vec(&self.wallets).context("save serialize json failed")?;
        fs::write(&self.f_path, data)
            .with_context(|| format!("save write json file failed: {:?}", self.f_path))?;
        Ok(())
    }

    pub fn list(&self) -> Result<Vec<WalletInfo>> {
        let mut ret = Vec::with_capacity(self.wallets.len());
        for w in self.wallets.iter() {
            ret.push(WalletInfo {
                name: w.name.clone(),
                eth_compatible_address: types::Address::from_base64(&w.address)
                    .with_context(|| format!("list::from_base64 failed: {}", w.address))?
                    .to_eth()
                    .with_context(|| format!("list::to_eth failed: {}", w.address))?,
                fra_address: types::Address::from_base64(&w.address)
                    .with_context(|| format!("list::from_base64 failed: {}", w.address))?
                    .to_bech32()
                    .with_context(|| format!("list::to_bech32 failed: {}", w.address))?,
            })
        }

        Ok(ret)
    }

    pub fn read(&self) -> ReadBuilder {
        ReadBuilder::new(self)
    }

    pub fn delete(&mut self, address: &str) -> Result<()> {
        let addr = detect_address(address).context("delete detect_address failed")?;
        self.wallets.retain(|w| w.address != addr);
        self.save()
            .with_context(|| format!("delete on save failed: {}", address))?;
        Ok(())
    }
}

pub fn detect_address(address: &str) -> Result<String> {
    if address.starts_with("0x") {
        Ok(types::Address::from_eth(address)
            .with_context(|| format!("detect_address::from_eth failed: {}", address))?
            .to_base64()
            .with_context(|| format!("detect_address::to_base64 failed: {}", address))?)
    } else if address.starts_with("fra1") {
        Ok(types::Address::from_bech32(address)
            .with_context(|| format!("detect_address::from_bech32 failed: {}", address))?
            .to_base64()
            .with_context(|| format!("detect_address::to_base64 failed: {}", address))?)
    } else {
        bail!("detect_address unknown address format: {}", address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_entry_wallet_read_builder() {
        let home = TempDir::new("test_entry_wallet_read_builder").unwrap();
        let mut wallets = Wallets::new(home.path()).unwrap();

        let wallet = Wallet {
            name: Some("wallet_test".to_string()),
            mnemonic: "some_mnemonic_1".to_string(),
            address: "B/2kcd30AtqMPEyTEO/I3fzVF+o=".to_string(),
            public: "oNveVIDjJWbNA6BzyqN30KzyZtC61Sbu0oV553JiD1g=".to_string(),
            secret: "/J3/PTVi3F642FIN4giagoo+kKgusZCKbg9mlE6IZds=".to_string(),
        };
        assert!(wallets.create(&wallet).is_ok());

        let got = wallets
            .read()
            .from_address("0x07fda471ddf402da8c3c4c9310efc8ddfcd517ea")
            .build()
            .unwrap();
        assert_eq!(got.public, wallet.public);

        let got = wallets
            .read()
            .from_address("fra11ql76guwa7spd4rpufjf3pm7gmh7d29l249t3dr")
            .build()
            .unwrap();
        assert_eq!(got.public, wallet.public);

        let got = wallets
            .read()
            .from_secret("/J3/PTVi3F642FIN4giagoo+kKgusZCKbg9mlE6IZds=")
            .build()
            .unwrap();
        assert_eq!(got.public, wallet.public);
    }

    #[test]
    fn test_entry_wallet_to_addresses() {
        let wallet = Wallet {
            name: Some("wallet_test".to_string()),
            mnemonic: "some_mnemonic_1".to_string(),
            address: "B/2kcd30AtqMPEyTEO/I3fzVF+o=".to_string(),
            public: "oNveVIDjJWbNA6BzyqN30KzyZtC61Sbu0oV553JiD1g=".to_string(),
            secret: "/J3/PTVi3F642FIN4giagoo+kKgusZCKbg9mlE6IZds=".to_string(),
        };

        assert_eq!(
            "0x07fda471ddf402da8c3c4c9310efc8ddfcd517ea",
            wallet.to_eth_address().unwrap()
        );
        assert_eq!(
            "fra11ql76guwa7spd4rpufjf3pm7gmh7d29l249t3dr",
            wallet.to_fra_address().unwrap()
        )
    }

    #[test]
    fn test_entry_wallet() {
        let home = TempDir::new("test_wallets_crud").unwrap();
        let mut wallets = Wallets::new(home.path()).unwrap();
        assert_eq!(wallets.list().unwrap().len(), 0);

        let wallet_1 = Wallet {
            name: Some("wallet_1".to_string()),
            mnemonic: "some_mnemonic_1".to_string(),
            address: "B/2kcd30AtqMPEyTEO/I3fzVF+o=".to_string(),
            public: "oNveVIDjJWbNA6BzyqN30KzyZtC61Sbu0oV553JiD1g=".to_string(),
            secret: "/J3/PTVi3F642FIN4giagoo+kKgusZCKbg9mlE6IZds=".to_string(),
        };

        assert!(wallets.create(&wallet_1).is_ok());
        assert_eq!(wallets.list().unwrap().len(), 1);
        let got = wallets
            .read()
            .from_address("0x07fda471ddf402da8c3c4c9310efc8ddfcd517ea")
            .build()
            .unwrap();
        assert_eq!(wallet_1, got);
        assert!(wallets
            .read()
            .from_address("not_exists_address")
            .build()
            .is_err());

        let wallet_2 = Wallet {
            name: None,
            mnemonic: "some_mnemonic_2".to_string(),
            address: "eOe5026+eoSJi5EF2YnUOda1ats=".to_string(),
            public: "lkucMadbGAy82AgGcqpg1Bh6iU732Ht6Ybfov99k7rw=".to_string(),
            secret: "P2cW1GEzeyuWrxzqTgZy+rYNeTabpUh9MTV7Upth2B0=".to_string(),
        };
        assert!(wallets.create(&wallet_2).is_ok());
        assert_eq!(wallets.list().unwrap().len(), 2);
        assert!(wallets
            .delete("0x07fda471ddf402da8c3c4c9310efc8ddfcd517ea")
            .is_ok());
        assert_eq!(wallets.list().unwrap().len(), 1);

        let got = wallets
            .read()
            .from_address("0x78e7b9d36ebe7a84898b9105d989d439d6b56adb")
            .build()
            .unwrap();
        assert_eq!(wallet_2, got);
    }
}
