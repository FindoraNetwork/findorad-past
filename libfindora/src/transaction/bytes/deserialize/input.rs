use primitive_types::H512;

use crate::{
    error::{convert_capnp_error, convert_capnp_noinschema, convert_try_slice_error},
    transaction::{Input, InputOperation},
    transaction_capnp::input,
};

pub fn from_input(input: input::Reader) -> abcf::Result<Input> {
    let txid = {
        let txid_slice = input.get_txid().map_err(convert_capnp_error)?;
        let inner = txid_slice.try_into().map_err(convert_try_slice_error)?;
        H512(inner)
    };

    let n = input.get_n();
    let operation = match input.get_operation().map_err(convert_capnp_noinschema)? {
        input::Operation::IssueAsset => InputOperation::IssueAsset,
        input::Operation::TransferAsset => InputOperation::TransferAsset,
        input::Operation::Undelegate => InputOperation::Undelegate,
        input::Operation::ClaimReward => InputOperation::ClaimReward,
    };

    Ok(Input { txid, n, operation })
}
