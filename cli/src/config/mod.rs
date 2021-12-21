mod node;
use node::Node;

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG_FILE: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub node: Node,
    #[serde(skip)]
    config_path: PathBuf,
}

impl Config {
    pub fn load(home_path: &Path) -> Result<Config> {
        if !home_path.exists() {
            fs::create_dir_all(home_path)
                .with_context(|| format!("failed to create_dir_all: {:?}", home_path))?;
        }

        let cfg_path = home_path.join(DEFAULT_CONFIG_FILE);
        if !cfg_path.exists() {
            let data = toml::to_string_pretty(&Config::default())
                .context("toml to_string_pretty failed")?;
            fs::write(&cfg_path, data)
                .with_context(|| format!("write config file failed: {:?}", cfg_path))?;
        }
        let data = fs::read_to_string(&cfg_path)
            .with_context(|| format!("failed to read_to_string: {:?}", cfg_path))?;

        let mut cfg: Config = toml::from_str(&data).context("toml from_str failed")?;
        cfg.config_path = cfg_path.clone();

        Ok(cfg)
    }

    pub fn save(&self) -> Result<()> {
        let data = toml::to_string_pretty(self).context("toml to_string_pretty failed")?;
        fs::write(&self.config_path, data).context("write config file failed")
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
        assert!(Config::load(&Path::new("/not-exist")).is_err());
    }

    #[test]
    fn test_config_new() {
        let home_path = TempDir::new("test_config").unwrap();
        let cfg = Config::default();
        assert!(cfg.save().is_ok());
        let cfg = Config::load(home_path.path()).unwrap();
        assert_eq!(cfg.node.address, "http://localhost:25576".to_string());
    }
}
