use zei::{
    chaum_pedersen::{ChaumPedersenProof, ChaumPedersenProofX},
    ristretto::{CompressedRistretto, RistrettoPoint, RistrettoScalar},
    serialization::ZeiFromToBytes,
    xfr::structs::{AssetTypeAndAmountProof, XfrRangeProof},
};

use crate::{
    error::{convert_capnp_error, convert_capnp_noinschema, convert_ruc_error},
    transaction_capnp::{chaum_pedersen_proof, range_proof, transaction::proof},
};

pub fn from_proof(reader: proof::Reader) -> abcf::Result<AssetTypeAndAmountProof> {
    let proof = {
        match reader.which().map_err(convert_capnp_noinschema)? {
            proof::Which::AssetMix(bytes) => {
                let r1cs = bulletproofs::r1cs::R1CSProof::zei_from_bytes(
                    bytes.map_err(convert_capnp_error)?,
                )
                .map_err(convert_ruc_error)?;

                AssetTypeAndAmountProof::AssetMix(r1cs.into())
            }
            proof::Which::ConfidentialAmount(e) => {
                let reader = e.map_err(convert_capnp_error)?;

                AssetTypeAndAmountProof::ConfAmount(parse_range_proof(reader)?)
            }
            proof::Which::ConfidentialAsset(e) => {
                let reader = e.map_err(convert_capnp_error)?;

                let proof = if reader.len() == 1 {
                    let proof0 = parse_chaum_pederson_proof(reader.get(0))?;

                    ChaumPedersenProofX {
                        c1_eq_c2: proof0,
                        zero: None,
                    }
                } else if reader.len() == 2 {
                    let proof0 = parse_chaum_pederson_proof(reader.get(0))?;
                    let proof1 = parse_chaum_pederson_proof(reader.get(1))?;
                    ChaumPedersenProofX {
                        c1_eq_c2: proof0,
                        zero: Some(proof1),
                    }
                } else {
                    return Err(abcf::Error::ABCIApplicationError(
                        90005,
                        String::from("parse error, chaum_pedersen_proof_x must have 1 or 2 proof."),
                    ));
                };

                AssetTypeAndAmountProof::ConfAsset(Box::new(proof))
            }
            proof::Which::ConfidentialAll(e) => {
                let reader = e.map_err(convert_capnp_error)?;

                let range_proof_reader = reader.get_amount().map_err(convert_capnp_error)?;

                let range_proof = parse_range_proof(range_proof_reader)?;

                let cpc_reader = reader.get_asset().map_err(convert_capnp_error)?;

                let cpc_proof = if cpc_reader.len() == 1 {
                    let proof0 = parse_chaum_pederson_proof(cpc_reader.get(0))?;

                    ChaumPedersenProofX {
                        c1_eq_c2: proof0,
                        zero: None,
                    }
                } else if cpc_reader.len() == 2 {
                    let proof0 = parse_chaum_pederson_proof(cpc_reader.get(0))?;
                    let proof1 = parse_chaum_pederson_proof(cpc_reader.get(1))?;
                    ChaumPedersenProofX {
                        c1_eq_c2: proof0,
                        zero: Some(proof1),
                    }
                } else {
                    return Err(abcf::Error::ABCIApplicationError(
                        90005,
                        String::from("parse error, chaum_pedersen_proof_x must have 1 or 2 proof."),
                    ));
                };

                AssetTypeAndAmountProof::ConfAll(Box::new((range_proof, cpc_proof)))
            }
            proof::Which::NoProof(_) => AssetTypeAndAmountProof::NoProof,
        }
    };
    Ok(proof)
}
fn parse_range_proof(reader: range_proof::Reader) -> abcf::Result<XfrRangeProof> {
    let range_proof = bulletproofs::RangeProof::zei_from_bytes(
        reader.get_range_proof().map_err(convert_capnp_error)?,
    )
    .map_err(convert_ruc_error)?;

    let xfr_diff_commitment_low = CompressedRistretto::zei_from_bytes(
        reader
            .get_diff_commitment_low()
            .map_err(convert_capnp_error)?,
    )
    .map_err(convert_ruc_error)?;

    let xfr_diff_commitment_high = CompressedRistretto::zei_from_bytes(
        reader
            .get_diff_commitment_high()
            .map_err(convert_capnp_error)?,
    )
    .map_err(convert_ruc_error)?;

    Ok(XfrRangeProof {
        range_proof,
        xfr_diff_commitment_low,
        xfr_diff_commitment_high,
    })
}

fn parse_chaum_pederson_proof(
    reader: chaum_pedersen_proof::Reader,
) -> abcf::Result<ChaumPedersenProof> {
    let c3 = RistrettoPoint::zei_from_bytes(reader.get_c3().map_err(convert_capnp_error)?)
        .map_err(convert_ruc_error)?;
    let c4 = RistrettoPoint::zei_from_bytes(reader.get_c4().map_err(convert_capnp_error)?)
        .map_err(convert_ruc_error)?;
    let z1 = RistrettoScalar::zei_from_bytes(reader.get_z1().map_err(convert_capnp_error)?)
        .map_err(convert_ruc_error)?;
    let z2 = RistrettoScalar::zei_from_bytes(reader.get_z2().map_err(convert_capnp_error)?)
        .map_err(convert_ruc_error)?;
    let z3 = RistrettoScalar::zei_from_bytes(reader.get_z3().map_err(convert_capnp_error)?)
        .map_err(convert_ruc_error)?;
    Ok(ChaumPedersenProof { c3, c4, z1, z2, z3 })
}
