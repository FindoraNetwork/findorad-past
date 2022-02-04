use wasmi::{ModuleImportResolver, Signature, FuncRef, Error, ValueType};

pub struct Resolver;

pub const INDEX_SET_REWARDS: usize = 1;
pub const INDEX_GET_PROPOSER: usize = 2;
pub const INDEX_GET_VALIDATOR_LIST: usize = 3;
pub const INDEX_GET_DELEGATION_LIST: usize = 4;
pub const INDEX_GET_DELEGATION_AMOUNT: usize = 5;

impl Resolver {
    fn check_signature(
        &self,
        index: usize,
        signature: &Signature
    ) -> bool {
        let (params, ret_ty): (&[ValueType], Option<ValueType>) = match index {
            ADD_FUNC_INDEX => (&[ValueType::I32, ValueType::I32], Some(ValueType::I32)),
            _ => return false,
        };
        signature.params() == params && signature.return_type() == ret_ty
    }
}

// impl ModuleImportResolver for Resolver {
//     fn resolve_func(
//         &self,
//         field_name: &str,
//         signature: &Signature
//     ) -> Result<FuncRef, Error> {
//         let index = match field_name {
//             "set_rewards" => INDEX_SET_REWARDS,
//             "get_proposer" => INDEX_GET_PROPOSER,
//             "get_validator_list" => INDEX_GET_DELEGATION_LIST,
//             "get_delegation_amount" => INDEX_GET_DELEGATION_AMOUNT,
//         };
//     }
// }
//
