use serde::{Deserialize, Serialize};
use zei::serialization::ZeiFromToBytes;
use zei::xfr::structs::BlindAssetRecord;
use abcf::module::EventValue;

#[derive(Clone, Debug, Deserialize, Serialize, abcf::Event)]
pub struct SendEvent {
    pub_key: String,
    send_amount: Option<u64>,
}

impl SendEvent {
    pub fn new_from_record(record: &BlindAssetRecord) -> Self {
        let base64_pub_key = base64::encode(&record.public_key.zei_to_bytes());
        let send_amount = record.amount.get_amount();
        Self {
            pub_key: base64_pub_key,
            send_amount
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, abcf::Event)]
pub struct RecvEvent {
    pub_key: String,
    recv_amount: Option<u64>,
}

impl RecvEvent {
    pub fn new_from_record(record: &BlindAssetRecord) -> Self {
        let base64_pub_key = base64::encode(&record.public_key.zei_to_bytes());
        let recv_amount = record.amount.get_amount();
        Self {
            pub_key: base64_pub_key,
            recv_amount
        }
    }
}

