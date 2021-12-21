use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
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

pub struct ListWallet {
    pub name: Option<String>,
    pub address: String,
}

pub struct Wallets {
    wallets: Vec<Wallet>,
    f_path: PathBuf,
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

    pub fn list(&self) -> Vec<ListWallet> {
        self.wallets
            .iter()
            .map(|w| ListWallet {
                name: w.name.clone(),
                address: w.address.clone(),
            })
            .collect()
    }

    pub fn read(&self, address: &str) -> Result<Wallet> {
        let result = self.wallets.iter().find(|w| w.address == address);
        match result {
            Some(w) => Ok(w.clone()),
            None => bail!("read connot find address: {}", address),
        }
    }

    pub fn delete(&mut self, address: &str) -> Result<()> {
        self.wallets.retain(|w| w.address != address);
        self.save()
            .with_context(|| format!("delete on save failed: {}", address))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_entry_wallet() {
        let home = TempDir::new("test_wallets_crud").unwrap();
        let mut wallets = Wallets::new(home.path()).unwrap();
        assert_eq!(wallets.list().len(), 0);

        let wallet_1 = Wallet {
            name: Some("wallet_1".to_string()),
            mnemonic: "some_mnemonic_1".to_string(),
            address: "some_address_1".to_string(),
            public: "some_public_1".to_string(),
            secret: "some_secret_1".to_string(),
        };

        assert!(wallets.create(&wallet_1).is_ok());
        assert_eq!(wallets.list().len(), 1);
        let got = wallets.read(&wallet_1.address).unwrap();
        assert_eq!(wallet_1, got);
        assert!(wallets.read("not_exists_wallet").is_err());

        let wallet_2 = Wallet {
            name: None,
            mnemonic: "some_mnemonic_2".to_string(),
            address: "some_address_2".to_string(),
            public: "some_public_2".to_string(),
            secret: "some_secret_2".to_string(),
        };
        assert!(wallets.create(&wallet_2).is_ok());
        assert_eq!(wallets.list().len(), 2);
        assert!(wallets.delete(&wallet_1.address).is_ok());
        assert_eq!(wallets.list().len(), 1);

        let got = wallets.read(&wallet_2.address).unwrap();
        assert_eq!(wallet_2, got);
    }
}
