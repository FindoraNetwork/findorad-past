use ethereum::{TransactionV2, LegacyTransactionMessage, TransactionAction};
use libfindora::Address;
use primitive_types::{H160, H256};
use rlp::{Decodable, Rlp};
use sha3::{Digest, Keccak256};

use crate::{transaction::EvmTransaction, Result, Error};

use super::crypto::recover_address;

pub fn convert_from_ethereum_tx(bytes: &[u8]) -> Result<EvmTransaction> {
    let rlp = Rlp::new(bytes);

    let etx = match TransactionV2::decode(&rlp)? {
        TransactionV2::Legacy(tx) => tx,
        _ => return Err(Error::OnlySupportLegacyTransaction),
    };

    let msg = LegacyTransactionMessage::from(etx.clone()).hash().0;

    let signature = &etx.signature;

    let pubkey = recover_address(signature, &msg)?;

    let mut res = [0u8; 64];
    res.copy_from_slice(&pubkey.serialize()[1..65]);

    let eaddr = H160::from(H256::from_slice(&Keccak256::digest(&res)));

    let chain_id = etx.signature.chain_id();
    let from = Some(Address::Eth(eaddr));
    // let from_output = None;
    let nonce = etx.nonce;
    let data = etx.input;
    let gas_limit = etx.gas_limit;

    match etx.action {
        TransactionAction::Create => {},
        TransactionAction::Call(e) => {},
    }

    // let to = Address::Eth()

    // let eaddr =

    Ok(Default::default())
}
