use std::fmt::Display;

use crate::config::Config;

use anyhow::{bail, Result};
use clap::Parser;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RPC_ADDR_REGEX: Regex =
        Regex::new(r"^(?:http(s)?://)?[\w.-]+(?:\.[\w\.-]+)*:\d*").unwrap();
}

#[derive(Parser, Debug)]
pub struct Command {
    #[clap(short, long)]
    /// Set the address of Findora rpc server
    set_rpc_server_address: Option<String>,
}

impl Command {
    pub fn execute(&self, config: Config) -> Result<Box<dyn Display>> {
        let mut config = config;

        if let Some(addr) = &self.set_rpc_server_address {
            if !is_address_validate(addr) {
                bail!("address is not valid: {}", addr)
            }
            config.node.address = addr.clone();
            config.save()?;
        }

        Ok(Box::new(0))
    }
}

// using a simple regex checking instead of using url crate,
// because
// 1. the rpc address setup comes only in here and it's format is simple
// 2. regex crate can be re-used in other places but url crate is not
fn is_address_validate(addr: &str) -> bool {
    RPC_ADDR_REGEX.is_match(addr)
}
