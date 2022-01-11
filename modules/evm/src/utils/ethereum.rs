use rlp::{Rlp, Decodable};

use crate::{Result, transaction::EvmTransaction};

pub fn convert_to_ethereum_tx(bytes: &[u8]) -> Result<EvmTransaction> {
    let rlp = Rlp::new(bytes);

    let etx = ethereum::TransactionV2::decode(&rlp)?;

    // let address =
}
