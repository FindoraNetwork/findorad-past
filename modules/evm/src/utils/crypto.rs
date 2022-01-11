use libsecp256k1::PublicKey;

use crate::Result;

pub fn recover_address(
    signature: &ethereum::TransactionSignature,
    msg: &[u8; 32],
) -> Result<PublicKey> {
    let mut rs = [0u8; 64];

    let r = signature.r();
    let s = signature.s();
    let v = signature.standard_v();

    let recovery_id = libsecp256k1::RecoveryId::parse(if v > 26 { v - 27 } else { v })?;

    rs[0..32].copy_from_slice(&r.0);
    rs[32..64].copy_from_slice(&s.0);

    let signature = libsecp256k1::Signature::parse_standard(&rs)?;

    let message = libsecp256k1::Message::parse(msg);

    let pubkey = libsecp256k1::recover(&message, &signature, &recovery_id)?;

    Ok(pubkey)
}
