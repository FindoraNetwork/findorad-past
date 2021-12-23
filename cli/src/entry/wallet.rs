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
    #[serde(default)]
    pub current: bool,
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

    // O(n) solution, we need to loop all wallets for fixing the uncorrectly changes.
    pub fn set_current(&mut self, address: &str) -> Result<()> {
        let mut found = false;
        let mut loc = 0;
        let mut unreasonables = Vec::with_capacity(self.wallets.len());

        for i in 0..self.wallets.len() {
            if self.wallets[i].address == address {
                found = true;
                loc = i;
            } else if self.wallets[i].address != address && self.wallets[i].current {
                unreasonables.push(i);
            }
        }

        if !found {
            bail!("set_current connot find address: {}", address);
        }

        for u in unreasonables {
            self.wallets[u].current = false;
        }

        self.wallets[loc].current = true;
        self.save()
            .with_context(|| format!("set_current on save failed: {}", address))?;
        Ok(())
    }

    pub fn get_current(&self) -> Result<Wallet> {
        let result = self.wallets.iter().find(|w| w.current);
        match result {
            Some(w) => Ok(w.clone()),
            None => bail!("get_current connot find the current one"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    fn helper_setup_wallets(wallets: &mut Wallets, max: usize) {
        for i in 0..max {
            wallets
                .create(&Wallet {
                    name: None,
                    mnemonic: format!("some_mnemonic_{}", i),
                    address: format!("some_address_{}", i),
                    public: format!("some_public_{}", i),
                    secret: format!("some_secret_{}", i),
                    current: false,
                })
                .unwrap()
        }
    }

    #[test]
    fn test_get_set_entry_wallet() {
        let home = TempDir::new("test_get_set_entry_wallet_happy_case").unwrap();
        let mut wallets = Wallets::new(home.path()).unwrap();
        helper_setup_wallets(&mut wallets, 3);

        // // no current wallet setted yet
        assert!(wallets.get_current().is_err());

        let want_address = "some_address_1";
        assert!(wallets.set_current(want_address).is_ok());
        let got = wallets.get_current().unwrap();
        assert_eq!(want_address, got.address);
        assert!(got.current);
        assert_eq!(1, wallets.wallets.iter().filter(|w| w.current).count());

        let want_address = "some_address_0";
        assert!(wallets.set_current(want_address).is_ok());
        let got = wallets.get_current().unwrap();
        assert_eq!(want_address, got.address);
        assert!(got.current);
        assert_eq!(1, wallets.wallets.iter().filter(|w| w.current).count());
    }

    #[test]
    fn test_entry_wallet_normal_crud() {
        let home = TempDir::new("test_wallets_crud").unwrap();
        let mut wallets = Wallets::new(home.path()).unwrap();
        assert_eq!(wallets.list().len(), 0);

        let wallet_1 = Wallet {
            name: Some("wallet_1".to_string()),
            mnemonic: "some_mnemonic_1".to_string(),
            address: "some_address_1".to_string(),
            public: "some_public_1".to_string(),
            secret: "some_secret_1".to_string(),
            current: false,
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
            current: false,
        };
        assert!(wallets.create(&wallet_2).is_ok());
        assert_eq!(wallets.list().len(), 2);
        assert!(wallets.delete(&wallet_1.address).is_ok());
        assert_eq!(wallets.list().len(), 1);

        let got = wallets.read(&wallet_2.address).unwrap();
        assert_eq!(wallet_2, got);
    }
}
