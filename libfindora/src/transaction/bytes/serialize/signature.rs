use crate::{transaction::Signature, transaction_capnp::signature, Result};
use zei::serialization::ZeiFromToBytes;

pub fn build_signature(signature: &Signature, builder: signature::Builder) -> Result<()> {
    match signature {
        Signature::Fra(s) => {
            let mut b = builder.init_fra();
            b.set_public_key(s.public_key.zei_to_bytes().as_ref());
            b.set_siganture(s.signature.zei_to_bytes().as_ref());
        }
    }

    Ok(())
}
