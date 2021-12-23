use abcf::bs3::MapStore;
use libfindora::{
    utxo::{Output, OutputId},
    Address,
};

use crate::Result;

pub fn mint(
    outputs_set: &mut impl MapStore<OutputId, Output>,
    owned_outputs: &mut impl MapStore<Address, Vec<OutputId>>,
    outputs: &[(OutputId, Output)],
) -> Result<()> {
    for (id, output) in outputs {
        outputs_set.insert(id.clone(), output.clone())?;
        if let Some(v) = owned_outputs.get_mut(&output.address)? {
            v.push(id.clone());
        } else {
            let v = vec![id.clone()];
            owned_outputs.insert(output.address.clone(), v)?;
        }
    }
    Ok(())
}
