use abcf_sdk::providers::Provider;
use libfindora::{
    utxo::{Output, OutputId},
    Address,
};

use crate::Result;

pub async fn get_owned_outputs<P: Provider>(
    _provider: &mut P,
    _address: &Address,
) -> Result<(Vec<OutputId>, Vec<Output>)> {
    Ok((Vec::new(), Vec::new()))
}
