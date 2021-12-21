use crate::{memo_capnp::memo, Result, transaction::Memo};

pub fn from_memo(reader: memo::Reader) -> Result<Vec<Memo>> {
    let mut res = Vec::new();

    for ethereum in reader.get_ethereum()?.iter() {
        let data = ethereum.get_data()?;

        let memo = Memo::Ethereum(data.to_vec());

        res.push(memo);
    }


    Ok(res)
}
