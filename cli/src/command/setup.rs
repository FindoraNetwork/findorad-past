use std::fmt::Display;

use anyhow::{bail, Context, Result};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RPC_ADDR_REGEX: Regex =
        Regex::new(r"(?:https?)://[-a-zA-Z0-9.]+:?([0-9]+)?").unwrap();
}

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Set the address of Findora rpc server
    set_rpc_server_address: Option<String>,
}

impl Command {
    pub fn execute(&self, config: &mut crate::config::Config) -> Result<Box<dyn Display>> {
        let mut result = vec![];

        if let Some(addr) = &self.set_rpc_server_address {
            if !is_address_validate(addr) {
                bail!("address is not valid: {}", addr)
            }
            result.push((
                "RPC Server Address".to_string(),
                config.node.address.clone(),
                addr.clone(),
            ));
            config.node.address = addr.clone();
        }

        config.save().context("save setup configs failed")?;
        Ok(Box::new(crate::display::setup::Display::from(result)))
    }
}

// using a simple regex checking instead of using url crate,
// because
// 1. the rpc address setup comes only in here and it's format is simple
// 2. regex crate can be re-used in other places but url crate is not
fn is_address_validate(addr: &str) -> bool {
    RPC_ADDR_REGEX.is_match(addr)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::utils::test_utils::TempDir;

    #[test]
    fn test_command_setup_execute_set_rpc_server_address() {
        let home_path = TempDir::new("test_command_setup_execute_set_rpc_server_address").unwrap();
        let mut cfg = Config::load(home_path.path()).unwrap();

        let mut cmd = Command {
            set_rpc_server_address: None,
        };
        assert!(cmd.execute(&mut cfg).is_ok());
        cmd.set_rpc_server_address = Some("http://localhost:9090".to_string());
        assert!(cmd.execute(&mut cfg).is_ok());
        cmd.set_rpc_server_address = Some("invalid url".to_string());
        assert!(cmd.execute(&mut cfg).is_err());
    }
}
