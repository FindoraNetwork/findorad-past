use abcf::manager::CallEntry;
use libfindora::utxo::Output;
use primitive_types::H512;

pub struct ArgAddUtxo {
    pub txid: H512,
    pub n: u32,
    pub output: Output,
}

impl ArgAddUtxo {
    pub fn to_call_entry(self) -> CallEntry {
        let args = Box::new(self);
        CallEntry {
            method: String::from("add_utxo"),
            args,
        }
    }
}
