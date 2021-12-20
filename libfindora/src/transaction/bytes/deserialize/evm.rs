use primitive_types::H256;

use crate::{
    evm::{Action, Create2, Evm},
    evm_capnp::evm_call, Result,
};

pub fn from_evm(input: evm_call::Reader) -> Result<Evm> {
    let nonce = input.get_nonce();
    let gas_limit = input.get_gas_limit();
    let data = input.get_data()?.to_vec();

    let action = match input.get_action().which()? {
        evm_call::action::Which::Call(_) => Action::Call,
        evm_call::action::Which::Create(_) => Action::Create,
        evm_call::action::Which::Create2(a) => {
            let input = a?;

            let salt = H256::from_slice(input.get_salt()?);
            Action::Create2(Create2 { salt })
        }
    };

    Ok(Evm {
        nonce,
        gas_limit,
        data,
        action,
    })
}
