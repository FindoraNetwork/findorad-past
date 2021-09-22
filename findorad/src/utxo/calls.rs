use abcf::manager::CallEntry;
use zei::xfr::structs::BlindAssetRecord;

pub struct ArgAddUtxo {
    pub txid: Vec<u8>,
    pub n: u32,
    pub output: BlindAssetRecord,
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
