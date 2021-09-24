mod node;
use std::{
    fs::{create_dir_all, read_to_string, write},
    path::{Path, PathBuf},
};

pub use node::Node;

use ruc::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub node: Node,
    #[serde(skip)]
    config_path: PathBuf,
}

impl Config {
    pub fn load(home_path: &Path, config_path: &Path) -> Result<Self> {
        let c_path = if home_path != Path::new(concat!(env!("HOME"), "/.findora/fn")) {
            let p = home_path.join("config.toml");
            p
        } else {
            config_path.to_path_buf()
        };

        let mut config = if c_path.exists() {
            let config_str = read_to_string(&c_path).c(d!())?;
            let mut config: Config = toml::from_str(&config_str).c(d!())?;

            config.node.home = home_path.to_path_buf();
            config
        } else {
            let config = Config::default();
            create_dir_all(home_path).c(d!())?;
            let data = toml::to_string_pretty(&config).c(d!())?;
            write(&c_path, data).c(d!())?;
            config
        };

        config.config_path = c_path;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let data = toml::to_string_pretty(self).c(d!())?;
        write(&self.config_path, data).c(d!())?;
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            node: Node::default(),
            config_path: PathBuf::from(concat!(env!("HOME"), "/.findora/fn/config.toml")),
        }
    }
}
