use std::str::FromStr;
use abcf_sdk::jsonrpc::endpoint::abci_query::Response;
use abcf_sdk::providers::Provider;
use primitive_types::H512;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use zei::xfr::structs::AssetType;
use libfindora::{
    utxo::{Output, OutputId},
    Address,
};

use crate::{Result, Error};
use crate::net::utils::abci_query;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OutputIdTemp {
    pub txid: String,
    pub n: u32,
}

impl OutputIdTemp {
    pub fn to_outputid(&self) -> Result<OutputId> {
        let h512 = H512::from_str(&self.txid).map_err(|_|Error::FraAddressFormatError)?;
        Ok(OutputId{ txid: h512, n: self.n })
    }
}

pub async fn get_owned_outputs<P: Provider>(
    provider: &mut P,
    address: &Address,
) -> Result<(Vec<OutputId>, Vec<Output>)> {
    let mut outputid_v = Vec::new();
    let mut output_v = Vec::new();
    let address_bytes = serde_json::to_vec(address)?;
    let hex_address = hex::encode(address_bytes);

    let path = format!("stateless/utxo/owned_outputs/0x{}",hex_address);
    let hex_path = "0x".to_string() + hex::encode(path.as_bytes().to_vec()).as_str();

    let params = serde_json::json!({
        "path": hex_path,
        "height": 0_i64,
    });

    let output_id_temps = abci_query::<Vec<OutputIdTemp>, P>(params, provider).await?;

    for output_id_temp in output_id_temps {
        let outputid = output_id_temp.to_outputid()?;
        let outputid_bytes = serde_json::to_vec(&outputid)?;
        let hex_outputid = hex::encode(outputid_bytes);

        let path = format!("stateful/utxo/outputs_set/0x{}",hex_outputid);
        let hex_path = "0x".to_string() + hex::encode(path.as_bytes().to_vec()).as_str();

        let params = serde_json::json!({
                "path": hex_path,
                "height": 0_i64,
            });
        let output = abci_query::<Output, P>(params, provider).await?;
        outputid_v.push(outputid);
        output_v.push(output);
    }
    Ok((outputid_v, output_v))
}
