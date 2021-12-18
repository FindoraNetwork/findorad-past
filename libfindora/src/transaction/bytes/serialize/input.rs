use crate::{
    transaction::{Input, InputOperation},
    transaction_capnp::input,
    Result,
};

pub fn build_input(input: &Input, builder: input::Builder) -> Result<()> {
    let mut builder = builder;

    builder.set_txid(input.txid.0.as_ref());
    builder.set_n(input.n);
    match input.operation {
        InputOperation::TransferAsset => builder.set_operation(input::Operation::TransferAsset),
        InputOperation::Undelegate => builder.set_operation(input::Operation::Undelegate),
        InputOperation::ClaimReward => builder.set_operation(input::Operation::ClaimReward),
    }

    Ok(())
}
