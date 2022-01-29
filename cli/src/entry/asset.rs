use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_ASSET_FILE: &str = "assets.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub address: String,
    pub memo: Option<String>,
    pub name: Option<String>,
    pub decimal_place: u8,
    pub maximun: Option<u64>,
    pub is_transferable: bool,
    pub is_issued: bool,
}

pub struct Assets {
    assets: Vec<Asset>,
    f_path: PathBuf,
}

impl Assets {
    pub fn new(home: &Path) -> Result<Assets> {
        let f_path = home.join(DEFAULT_ASSET_FILE);
        if !f_path.exists() {
            fs::write(f_path.clone(), "[]")?;
        }

        let data = fs::read(f_path.clone())
            .with_context(|| format!("new read json file failed: {:?}", f_path))?;
        let assets: Vec<Asset> = serde_json::from_slice(&data)
            .with_context(|| format!("new deserialize json failed: {:?}", f_path))?;

        Ok(Assets { assets, f_path })
    }

    pub fn create(&mut self, asset: &Asset) -> Result<()> {
        self.assets.push(asset.clone());
        let data = serde_json::to_vec(&self.assets).context("save serialize json failed")?;
        fs::write(&self.f_path, data)
            .with_context(|| format!("save write json file failed: {:?}", self.f_path))?;
        Ok(())
    }

    pub fn read(&self, addr: &str) -> Result<Asset> {
        match self.assets.iter().find(|a| a.address == addr) {
            Some(a) => Ok(a.clone()),
            None => bail!("read cannot find"),
        }
    }

    pub fn list(&self) -> Result<Vec<Asset>> {
        Ok(self.assets.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_entry_asset_create_read() {
        let home = TempDir::new("test_entry_asset_create_read").unwrap();
        let mut assets = Assets::new(home.path()).unwrap();
        assert_eq!(assets.list().unwrap().len(), 0);

        let asset = Asset {
            address: "0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string(),
            memo: Some("this is a test asset 1".to_string()),
            name: Some("TEST1".to_string()),
            decimal_place: 6,
            maximun: Some(9999999),
            is_transferable: true,
            is_issued: true,
        };

        assert!(assets.create(&asset).is_ok());
        assert_eq!(assets.list().unwrap().len(), 1);
        let got = assets
            .read("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117")
            .unwrap();
        assert_eq!(asset, got);
    }
}
