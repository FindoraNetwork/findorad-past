use primitive_types::H512;

use crate::{
    evm,
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
    let operation = match input.get_operation().which()? {
        input::operation::Which::TransferAsset(_) => InputOperation::TransferAsset,
        input::operation::Which::EvmCall(a) => {
            let reader = a?;

            let n = reader.get_n();

            InputOperation::EvmCall(evm::Input { n })
        }
    };

    Ok(Input { txid, n, operation })
}
