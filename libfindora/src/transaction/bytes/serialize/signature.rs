use crate::{transaction::Signature, transaction_capnp::signature, Address, Result};
use zei::serialization::ZeiFromToBytes;

pub fn build_signature(signature: &Signature, builder: signature::Builder) -> Result<()> {
    match signature {
        Signature::Fra(s) => {
            let mut b = builder.init_fra();

            b.set_public_key(s.public_key.zei_to_bytes().as_ref());
            b.set_siganture(s.signature.zei_to_bytes().as_ref());

            let mut builder = b.init_address();

            match &s.address {
                Address::Eth(a) => builder.set_eth(a.0.as_ref()),
                Address::Fra(a) => builder.set_fra(a.0.as_ref()),
                Address::BlockHole => builder.set_block_hole(()),
            }
        }
    }

    Ok(())
}
