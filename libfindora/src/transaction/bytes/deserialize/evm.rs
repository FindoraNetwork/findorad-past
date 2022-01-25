use primitive_types::H256;

use crate::{
    evm::{Action, Create2, Evm},
    evm_capnp::{action, output},
    Result,
};

pub fn from_evm(input: output::Reader) -> Result<Evm> {
    let nonce = input.get_nonce();
    let gas_limit = input.get_gas_limit();
    let data = input.get_data()?.to_vec();
    let chain_id = input.get_gas_limit();

    let action_reader = input.get_action()?;

    let action = match action_reader.which()? {
        action::Which::Call(_) => Action::Call,
        action::Which::Create(_) => Action::Create,
        action::Which::Create2(a) => {
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
        chain_id,
    })
}
