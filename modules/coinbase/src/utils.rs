use abcf::bs3::MapStore;
use libfindora::utxo::Output;

use crate::{Result, types::OutputChain};

pub fn mint(
    target_height: i64,
    output: Output,
    pending_outputs: &mut impl MapStore<i64, OutputChain>,
) -> Result<()> {

    let next = 0;

    let oc = OutputChain {
        output,
        next,
    };

    pending_outputs.insert(target_height, oc)?;

    Ok(())
}
