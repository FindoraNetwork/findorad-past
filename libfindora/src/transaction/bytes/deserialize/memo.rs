use crate::{evm::EvmMemo, memo_capnp::memo, transaction::Memo, Result};

pub fn from_memo(reader: memo::Reader) -> Result<Vec<Memo>> {
    let mut res = Vec::new();

    for ethereum in reader.get_ethereum()?.iter() {
        let data = ethereum.get_data()?;

        let n = ethereum.get_n();

        let memo = Memo::Ethereum(EvmMemo {
            tx: data.to_vec(),
            n,
        });

        res.push(memo);
    }

    Ok(res)
}
