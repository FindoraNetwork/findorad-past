use zei::{
    serialization::ZeiFromToBytes,
    xfr::sig::{XfrPublicKey, XfrSignature},
};

use crate::{
    transaction::{FraSignature, Signature},
    transaction_capnp::signature,
    Error, Result,
};

use super::output::from_address;

pub fn from_signature(reader: signature::Reader) -> Result<Signature> {
    let signature = match reader.which()? {
        signature::Fra(a) => {
            let reader = a?;

            let bytes = reader.get_address()?;
            let address = from_address(bytes)?;

            let bytes = reader.get_public_key()?;
            let public_key = XfrPublicKey::zei_from_bytes(bytes)?;

            let bytes = reader.get_siganture()?;
            let signature = XfrSignature::zei_from_bytes(bytes)?;

            Signature::Fra(FraSignature {
                address,
                public_key,
                signature,
            })
        }
        signature::None(_) => return Err(Error::Unknown),
    };

    Ok(signature)
}
