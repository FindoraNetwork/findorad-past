
use clap::{ArgGroup, Parser};
use ruc::*;
use zei::{
    serialization::ZeiFromToBytes,
    xfr::{
        sig::XfrPublicKey,
    }
};
use libfindora::common::AssetTypeCode;

use crate::{
    config::Config,
    utils::{account_to_keypair, read_account_list, get_owned_asset_list,
            get_asset_type_list, get_token_code_list},
};

#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("query"))]
pub struct Command {
    #[clap(short, long, group = "query")]
    /// Get asset list by base64 string of pubkey
    get_owned_asset: Option<String>,

    #[clap(long, group = "query")]
    get_asset_type: Option<String>,

    #[clap(long, group = "query")]
    get_token_code: Option<String>,

    #[clap(short = 'i', long)]
    account_index: Option<usize>,
}

impl Command {
    pub async fn execute(&self, config: Config) -> Result<()> {

        if let Some(pubkey_base64) = self.get_owned_asset.as_ref() {

            let xfr_pubkey = if let Some(index) = self.account_index {
                let v = read_account_list(&config).await?;
                if let Some(account) = v.get(index) {
                    let result = account_to_keypair(account)?;
                    result.pub_key
                } else {
                    return Err(Box::from(d!("index not exist")));
                }
            } else {
                let pubkey_bytes = base64::decode(pubkey_base64).c(d!())?;
                let xfr_pubkey = XfrPublicKey::zei_from_bytes(&pubkey_bytes)?;
                xfr_pubkey
            };

            let v = get_owned_asset_list(vec![xfr_pubkey]).await?;
            let json = serde_json::to_string(&v).c(d!())?;
            println!("{:#}",json);

        }

        if let Some(asset_type_code_base64) = self.get_asset_type.as_ref() {
            let atc = AssetTypeCode::new_from_base64(&*asset_type_code_base64)?;

            let v = get_asset_type_list(vec![atc]).await?;
            let json = serde_json::to_string(&v).c(d!())?;
            println!("{:#}",json);
        }

        if let Some(asset_type_code_base64) = self.get_token_code.as_ref() {
            let atc = AssetTypeCode::new_from_base64(&*asset_type_code_base64)?;

            let v = get_token_code_list(atc).await?;
            let json = serde_json::to_string(&v).c(d!())?;
            println!("{:#}",json);

        }


        Ok(())
    }
}