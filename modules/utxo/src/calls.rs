use abcf::manager::CallEntry;
use libfindora::utxo::Output;

pub struct ArgAddUtxo {
    pub txid: Vec<u8>,
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
