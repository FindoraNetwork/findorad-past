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

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::utils::test_utils::TempDir;
//
//     #[test]
//     fn test_entry_asset_create_read_update() {
//         let home = TempDir::new("test_entry_asset_create_read").unwrap();
//         let mut assets = Assets::new(home.path()).unwrap();
//         let address = "0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string();
//         let asset_type = base64::encode_config(&[9; ASSET_TYPE_LENGTH], base64::URL_SAFE);
//         let mut asset = Transfer::new_from_asset_type_base64(&asset_type).unwrap();
//         asset.address = address.clone();
//
//         assert!(assets.create(&asset).is_ok());
//         assert_eq!(assets.list(&address).len(), 1);
//         let got = assets.read(&address, &asset_type).unwrap();
//         assert_eq!(asset, got);
//
//         asset.name = Some("TEST1".to_string());
//         assert!(assets.update(&asset).is_ok());
//         assert_eq!(assets.list(&address).len(), 1);
//         let got = assets.read(&address, &asset_type).unwrap();
//         assert_eq!(asset, got);
//     }
// }
