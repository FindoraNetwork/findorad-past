use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use libfindora::asset::AssetType;
use rand_chacha::{rand_core::RngCore, rand_core::SeedableRng, ChaChaRng};
use serde::{Deserialize, Serialize};

const DEFAULT_ASSET_FILE: &str = "assets.json";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub address: String,
    pub asset_type: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    pub decimal_place: u8,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximun: Option<u64>,
    pub is_transferable: bool,
    pub is_issued: bool,
}

impl Default for Asset {
    fn default() -> Asset {
        let mut asset_type: [u8; 32] = [0; 32];
        let mut rng = ChaChaRng::from_entropy();
        rng.try_fill_bytes(&mut asset_type).unwrap_or_default();

        Asset {
            address: "".to_string(),
            asset_type: asset_type.to_vec(),
            memo: None,
            code: None,
            decimal_place: 6,
            maximun: None,
            is_transferable: false,
            is_issued: false,
        }
    }
}

impl Asset {
    pub fn new() -> Asset {
        Asset::default()
    }

    // TODO: this should be implemented in AssetType itself to impl TryFrom
    // to let itself can convert back into a fixed size array of u8,
    // then the entry of asset can simply store AssetType type as array_type to serve
    // display and command traits
    pub fn get_asset_type(&self) -> AssetType {
        let t: Box<[u8; 32]> = self
            .asset_type
            .clone()
            .into_boxed_slice()
            .try_into()
            .unwrap_or_default();
        AssetType(*t)
    }

    // TODO: this function too
    // then cli can remove the base64 dependency
    pub fn get_asset_type_base64(&self) -> String {
        base64::encode_config(self.asset_type.clone().into_boxed_slice(), base64::URL_SAFE)
    }
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
        self.save()
            .with_context(|| format!("create on save failed: {:?}", asset))?;
        Ok(())
    }

    fn save(&self) -> Result<()> {
        let data = serde_json::to_vec(&self.assets).context("save serialize json failed")?;
        fs::write(&self.f_path, data)
            .with_context(|| format!("save write json file failed: {:?}", self.f_path))?;
        Ok(())
    }

    pub fn update(&mut self, asset: &Asset) -> Result<()> {
        if let Some(i) = self.assets.iter().position(|a| a.address == asset.address) {
            self.assets[i] = asset.clone();
            self.save()
                .with_context(|| format!("update on save failed: {:?}", asset))?;
        }
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
    fn test_entry_asset_create_read_update() {
        let home = TempDir::new("test_entry_asset_create_read").unwrap();
        let mut assets = Assets::new(home.path()).unwrap();
        assert_eq!(assets.list().unwrap().len(), 0);

        let mut asset = Asset {
            address: "0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117".to_string(),
            asset_type: vec![9; 32],
            memo: Some("this is a test asset 1".to_string()),
            code: None,
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

        asset.code = Some("TEST1".to_string());
        assert!(assets.update(&asset).is_ok());
        let got = assets
            .read("0xf8d1fa7c6a8af4a78f862cac72fe05de0e308117")
            .unwrap();
        assert_eq!(asset, got);
    }
}
