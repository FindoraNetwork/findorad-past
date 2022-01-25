use crate::{memo_capnp::memo, transaction::Memo, Result};

pub fn build_memos(memos: &[Memo], builder: memo::Builder) -> Result<()> {
    let mut builder = builder;

    let mut ethereum = builder.reborrow().get_ethereum()?;

    // let memo_length = memos.len().try_into()?;

    for (index, memo) in memos.iter().enumerate() {
        let mut memo_builder = ethereum.reborrow().get(index.try_into()?);

        match memo {
            Memo::Ethereum(e) => {
                memo_builder.set_data(&e.tx);
                memo_builder.set_n(e.n);
            }
        }
    }
    Ok(())
}
