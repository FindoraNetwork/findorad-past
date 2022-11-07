use wasmi::{Error, FuncInstance, FuncRef, ModuleImportResolver, Signature};

pub struct Resolver;

pub const INDEX_LOG: usize = 1;
pub const INDEX_SET_REWARDS: usize = 2;
pub const INDEX_GET_PROPOSER: usize = 3;
pub const INDEX_GET_VALIDATOR_LIST_LEN: usize = 4;
pub const INDEX_GET_VALIDATOR_LIST_ITEM: usize = 5;
pub const INDEX_GET_DELEGATION_LIST_LEN: usize = 6;
pub const INDEX_GET_DELEGATION_LIST_ITEM: usize = 7;
pub const INDEX_GET_DELEGATION_AMOUNT: usize = 8;

// impl Resolver {
// fn check_signature(
//     &self,
//     index: usize,
//     signature: &Signature
// ) -> bool {
//     let (params, ret_ty): (&[ValueType], Option<ValueType>) = match index {
//         INDEX_SET_REWARDS => (&[ValueType::I32, ValueType::I32, ValueType::I64], None),
//         INDEX_GET_PROPOSER => (&[ValueType::I64, ValueType::I32], None),
//         INDEX_GET_VALIDATOR_LIST_LEN => (&[ValueType::I64], Some(ValueType::I32)),
//         INDEX_GET_VALIDATOR_LIST_ITEM => (&[ValueType::I64, ValueType::I32, ValueType::I32], Some(ValueType::I32)),
//         _ => return false,
//     };
//     signature.params() == params && signature.return_type() == ret_ty
// }
// }

impl ModuleImportResolver for Resolver {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        let index = match field_name {
            "log" => INDEX_LOG,
            "set_rewards" => INDEX_SET_REWARDS,
            "get_proposer" => INDEX_GET_PROPOSER,
            "get_validator_list_len" => INDEX_GET_VALIDATOR_LIST_LEN,
            "get_validator_list_item" => INDEX_GET_VALIDATOR_LIST_ITEM,
            "get_delegation_list_len" => INDEX_GET_DELEGATION_LIST_LEN,
            "get_delegation_list_item" => INDEX_GET_DELEGATION_LIST_ITEM,
            "get_delegation_amount" => INDEX_GET_DELEGATION_AMOUNT,
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {} not found",
                    field_name
                )))
            }
        };

        Ok(FuncInstance::alloc_host(signature.clone(), index))
    }
}
