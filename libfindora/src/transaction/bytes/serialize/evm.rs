use crate::{
    evm::{Action, Evm},
    evm_capnp::output,
    Result,
};

pub fn build_evm(evm: &Evm, builder: output::Builder) -> Result<()> {
    let mut builder = builder;

    builder.set_nonce(evm.nonce);
    builder.set_data(&evm.data);
    builder.set_chain_id(evm.chain_id);
    builder.set_gas_limit(evm.gas_limit);

    let mut action = builder.get_action()?;

    match &evm.action {
        Action::Call => action.set_call(()),
        Action::Create => action.set_create(()),
        Action::Create2(a) => {
            let mut builder = action.init_create2();

            builder.set_salt(a.salt.as_bytes());
        }
    }

    Ok(())
}
