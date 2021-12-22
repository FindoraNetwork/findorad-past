mod node;
pub use node::Node;

use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub node: Node,
    #[serde(skip)]
    config_path: PathBuf,
}

impl Config {
    pub fn new(home_path: &Path, config_path: &Path) -> Result<Config> {
        if !home_path.exists() {
            create_dir_all(home_path)
                .with_context(|| format!("failed to create_dir_all: {:?}", home_path))?;
        }

        let d = read_to_string(config_path)
            .with_context(|| format!("failed to read_to_string: {:?}", config_path))?;

        let mut cfg: Config = toml::from_str(&d).context("toml from_str failed")?;
        cfg.config_path = config_path.to_path_buf();

        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let data = toml::to_string_pretty(self).context("toml to_string_pretty failed")?;
        write(&self.config_path, data).context("write config file failed")
    }
}

impl Default for Config {
    fn default() -> Config {
        // must get "home"
        let mut cfg = home::home_dir().unwrap();
        cfg.push(".findora");
        cfg.push("fn");
        cfg.push("config.toml");

        Config {
            node: Node::default(),
            config_path: cfg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_config_new_home_path_not_exist() {
        assert!(Config::new(&Path::new("/not-exist"), &Path::new("not-exist")).is_err());
    }

    #[test]
    fn test_config_new_config_path_not_exist() {
        let home_path = TempDir::new("test_config_new_config_path_not_exist").unwrap();
        assert!(Config::new(home_path.path(), &Path::new("/not-exist")).is_err());
    }

    #[test]
    fn test_config_new() {
        let home_path = TempDir::new("test_config").unwrap();
        let config_path = home_path.path().join("config.toml");

        let cfg = Config {
            node: Node::default(),
            config_path: config_path.clone(),
        };
        assert!(cfg.save().is_ok());
        let cfg = Config::new(home_path.path(), &config_path).unwrap();
        assert_eq!(cfg.node.address, "http://localhost:25576".to_string());
    }
}
