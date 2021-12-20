use primitive_types::H512;

use crate::{
    transaction::{Input, InputOperation},
    transaction_capnp::input,
    Result,
};

pub fn from_input(input: input::Reader) -> Result<Input> {
    let txid = {
        let txid_slice = input.get_txid()?;
        let inner = txid_slice.try_into()?;
        H512(inner)
    };

    let n = input.get_n();
    let operation = match input.get_operation()? {
        input::Operation::TransferAsset => InputOperation::TransferAsset,
        input::Operation::EvmCall => InputOperation::EvmCall,
    };

    Ok(Input { txid, n, operation })
}
