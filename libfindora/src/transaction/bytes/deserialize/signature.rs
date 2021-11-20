use zei::{
    serialization::ZeiFromToBytes,
    xfr::sig::{XfrPublicKey, XfrSignature},
};

use crate::{
    error::{convert_capnp_error, convert_capnp_noinschema, convert_ruc_error, placeholder_error},
    transaction::{FraSignature, Signature},
    transaction_capnp::signature,
};

pub fn from_signature(reader: signature::Reader) -> abcf::Result<Signature> {
    let signature = match reader.which().map_err(convert_capnp_noinschema)? {
        signature::Fra(a) => {
            let reader = a.map_err(convert_capnp_error)?;
            let bytes = reader.get_public_key().map_err(convert_capnp_error)?;
            let public_key = XfrPublicKey::zei_from_bytes(bytes).map_err(convert_ruc_error)?;

            let bytes = reader.get_siganture().map_err(convert_capnp_error)?;
            let signature = XfrSignature::zei_from_bytes(bytes).map_err(convert_ruc_error)?;

            Signature::Fra(FraSignature {
                public_key,
                signature,
            })
        }
        signature::None(_) => return Err(placeholder_error()),
    };

    Ok(signature)
}
