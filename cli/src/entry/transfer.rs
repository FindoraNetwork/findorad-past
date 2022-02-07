use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_TRANSFER_FILE: &str = "transfers.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transfer {
    pub name: String,
    pub from_address: String,
    pub to_address: String,
    pub public_key: String,
    pub amount: u64,
    pub asset_type: String,
    pub is_confidential_amount: bool,
    pub is_confidential_asset: bool,
}

pub struct Transfers {
    transfers: HashMap<String, Vec<Transfer>>,
    f_path: PathBuf,
}

impl Transfers {
    pub fn new(home: &Path) -> Result<Transfers> {
        let f_path = home.join(DEFAULT_TRANSFER_FILE);
        if !f_path.exists() {
            fs::write(f_path.clone(), "{}")?;
        }

        let data = fs::read(f_path.clone())
            .with_context(|| format!("new read json file failed: {:?}", f_path))?;
        let transfers: HashMap<String, Vec<Transfer>> = serde_json::from_slice(&data)
            .with_context(|| format!("new deserialize json failed: {:?}", f_path))?;

        Ok(Transfers { transfers, f_path })
    }

    pub fn create(&mut self, transfer: &Transfer) -> Result<()> {
        self.transfers
            .entry(transfer.name.clone())
            .or_insert_with(Vec::new)
            .push(transfer.clone());
        self.save()
            .with_context(|| format!("create on save failed: {:?}", transfer))
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_string(&self.transfers).context("save serialize json failed")?;
        fs::write(&self.f_path, data)
            .with_context(|| format!("save write json file failed: {:?}", self.f_path))
    }

    pub fn read(&self, name: &str) -> Result<Vec<Transfer>> {
        Ok(self
            .transfers
            .get(name)
            .with_context(|| format!("read cannot find name: {}", name))?
            .clone())
    }

    pub fn delete(&mut self, name: &str) -> Result<()> {
        self.transfers.remove(name);
        self.save()
            .with_context(|| format!("delete on save failed: {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_entry_transfer_create_read_delete() {
        let home = TempDir::new("test_entry_transfer_create_read_delete").unwrap();
        let mut transfers = Transfers::new(home.path()).unwrap();

        let t1_1 = Transfer {
            name: "name_t1 ".to_string(),
            from_address: "from_address_t1_1 ".to_string(),
            to_address: "to_address_t1_1 ".to_string(),
            public_key: "public_key_t1_1 ".to_string(),
            amount: 99,
            asset_type: "asset_type_t1_1 ".to_string(),
            is_confidential_amount: true,
            is_confidential_asset: true,
        };
        let t1_2 = Transfer {
            name: "name_t1 ".to_string(),
            from_address: "from_address_t1_2 ".to_string(),
            to_address: "to_address_t1_2 ".to_string(),
            public_key: "public_key_t1_2 ".to_string(),
            amount: 99,
            asset_type: "asset_type_t1_2 ".to_string(),
            is_confidential_amount: true,
            is_confidential_asset: true,
        };

        transfers.create(&t1_1).unwrap();
        transfers.create(&t1_2).unwrap();
        assert_eq!(
            vec![t1_1.clone(), t1_2.clone()],
            transfers.read(&t1_1.name).unwrap()
        );

        let t2 = Transfer {
            name: "name_t2 ".to_string(),
            from_address: "from_address_t2 ".to_string(),
            to_address: "to_address_t2 ".to_string(),
            public_key: "public_key_t2 ".to_string(),
            amount: 9,
            asset_type: "asset_type_t2 ".to_string(),
            is_confidential_amount: false,
            is_confidential_asset: false,
        };
        transfers.create(&t2).unwrap();
        assert_eq!(vec![t2.clone()], transfers.read(&t2.name).unwrap());

        transfers.delete(&t2.name).unwrap();
        assert!(transfers.read(&t2.name).is_err());
    }
}
