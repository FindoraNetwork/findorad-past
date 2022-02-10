use crate::{
    transaction::{Input, InputOperation},
    transaction_capnp::input,
    Result,
};

pub fn build_input(input: &Input, builder: input::Builder) -> Result<()> {
    let mut builder = builder;

    builder.set_txid(input.txid.0.as_ref());
    builder.set_n(input.n);

    let mut operation = builder.init_operation();

    match &input.operation {
        InputOperation::TransferAsset => operation.set_transfer_asset(()),
    }

    Ok(())
}
