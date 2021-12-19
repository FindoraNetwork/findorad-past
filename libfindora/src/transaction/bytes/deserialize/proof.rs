use zei::{
    chaum_pedersen::{ChaumPedersenProof, ChaumPedersenProofX},
    ristretto::{CompressedRistretto, RistrettoPoint, RistrettoScalar},
    serialization::ZeiFromToBytes,
    xfr::structs::{AssetTypeAndAmountProof, XfrRangeProof},
};

use crate::{
    transaction_capnp::{chaum_pedersen_proof, range_proof, transaction::proof},
    Error, Result,
};

pub fn from_proof(reader: proof::Reader) -> Result<AssetTypeAndAmountProof> {
    let proof = {
        match reader.which()? {
            proof::Which::AssetMix(bytes) => {
                let r1cs = bulletproofs::r1cs::R1CSProof::zei_from_bytes(bytes?)?;

                AssetTypeAndAmountProof::AssetMix(r1cs.into())
            }
            proof::Which::ConfidentialAmount(e) => {
                let reader = e?;

                AssetTypeAndAmountProof::ConfAmount(parse_range_proof(reader)?)
            }
            proof::Which::ConfidentialAsset(e) => {
                let reader = e?;

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
                    return Err(Error::ChaumPedersenProofParseError);
                };

                AssetTypeAndAmountProof::ConfAsset(Box::new(proof))
            }
            proof::Which::ConfidentialAll(e) => {
                let reader = e?;

                let range_proof_reader = reader.get_amount()?;

                let range_proof = parse_range_proof(range_proof_reader)?;

                let cpc_reader = reader.get_asset()?;

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
                    return Err(Error::ChaumPedersenProofParseError);
                };

                AssetTypeAndAmountProof::ConfAll(Box::new((range_proof, cpc_proof)))
            }
            proof::Which::NoProof(_) => AssetTypeAndAmountProof::NoProof,
        }
    };
    Ok(proof)
}
fn parse_range_proof(reader: range_proof::Reader) -> Result<XfrRangeProof> {
    let range_proof = bulletproofs::RangeProof::zei_from_bytes(reader.get_range_proof()?)?;

    let xfr_diff_commitment_low =
        CompressedRistretto::zei_from_bytes(reader.get_diff_commitment_low()?)?;

    let xfr_diff_commitment_high =
        CompressedRistretto::zei_from_bytes(reader.get_diff_commitment_high()?)?;

    Ok(XfrRangeProof {
        range_proof,
        xfr_diff_commitment_low,
        xfr_diff_commitment_high,
    })
}

fn parse_chaum_pederson_proof(reader: chaum_pedersen_proof::Reader) -> Result<ChaumPedersenProof> {
    let c3 = RistrettoPoint::zei_from_bytes(reader.get_c3()?)?;
    let c4 = RistrettoPoint::zei_from_bytes(reader.get_c4()?)?;
    let z1 = RistrettoScalar::zei_from_bytes(reader.get_z1()?)?;
    let z2 = RistrettoScalar::zei_from_bytes(reader.get_z2()?)?;
    let z3 = RistrettoScalar::zei_from_bytes(reader.get_z3()?)?;
    Ok(ChaumPedersenProof { c3, c4, z1, z2, z3 })
}
