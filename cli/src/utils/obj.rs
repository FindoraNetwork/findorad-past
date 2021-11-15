use abcf::FromBytes;
use libfindora::transaction::Transaction;
use ruc::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Resp {
    pub code: i64,
    pub codespace: String,
    pub data: String,
    pub hash: String,
    pub log: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryRespResult {
    pub code: i64,
    pub codespace: String,
    pub data: String,
    pub events: Vec<Value>,
    pub gas_used: String,
    pub gas_wanted: String,
    pub info: String,
    pub log: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResp {
    pub hash: String,
    pub height: String,
    pub index: i64,
    pub tx: String,
    pub tx_result: QueryRespResult,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryValidator {
    pub address: String,
    pub pub_key: QueryVPK,
    pub voting_power: String,
    pub proposer_priority: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryVPK {
    #[serde(rename = "type")]
    pub ty: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct QueryValidators {
    pub validators: Vec<QueryValidator>,
    pub total: u64,
}

impl QueryResp {
    pub fn parse_tx(&mut self) -> Result<()> {
        let tx = self.tx.clone();

        let bytes = base64::decode(tx.as_bytes()).c(d!())?;

        let _tx = Transaction::from_bytes(&*bytes).map_err(|e| d!(e.message()))?;

        // self.tx = tx;
        Ok(())
    }
}
