use ethereum::TransactionV2;
use rlp::{Rlp, Decodable};

use crate::{memo_capnp::memo, Result, transaction::Memo};

pub fn from_memo(reader: memo::Reader) -> Result<Vec<Memo>> {
    let mut res = Vec::new();

    for ethereum in reader.get_ethereum()?.iter() {
        let data = ethereum.get_data()?;

        let rlp = Rlp::new(data);

        let tx = TransactionV2::decode(&rlp)?;

        let memo = Memo::Ethereum(tx);

        res.push(memo);
    }


    Ok(res)
}
