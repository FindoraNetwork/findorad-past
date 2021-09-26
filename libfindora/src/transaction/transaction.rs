use std::convert::TryInto;

use capnp::{message::ReaderOptions, serialize::read_message};
use ruc::*;
use serde::{Deserialize, Serialize};
use zei::{
    chaum_pedersen::{ChaumPedersenProof, ChaumPedersenProofX},
    ristretto::{CompressedRistretto, RistrettoPoint, RistrettoScalar},
    serialization::ZeiFromToBytes,
    xfr::{
        sig::XfrSignature,
        structs::{
            AssetType, AssetTypeAndAmountProof, BlindAssetRecord, XfrAmount, XfrAssetType,
            XfrRangeProof, ASSET_TYPE_LENGTH,
        },
    },
};

use crate::transaction_capnp;

use super::{Input, InputOperation, Output, OutputOperation};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub txid: Vec<u8>,
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
    pub proof: AssetTypeAndAmountProof,
    pub signatures: Vec<XfrSignature>,
}

impl abcf::Transaction for Transaction {}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            txid: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
            signatures: Vec::new(),
        }
    }
}

fn convert_capnp_error(e: capnp::Error) -> abcf::Error {
    abcf::Error::ABCIApplicationError(90001, format!("{:?}", e))
}

fn convert_capnp_noinschema(e: capnp::NotInSchema) -> abcf::Error {
    abcf::Error::ABCIApplicationError(90001, format!("{:?}", e))
}

fn convert_ruc_error(e: Box<dyn RucError>) -> abcf::Error {
    abcf::Error::ABCIApplicationError(90004, format!("{:?}", e))
}

fn parse_range_proof(
    reader: transaction_capnp::range_proof::Reader,
) -> abcf::Result<XfrRangeProof> {
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
    reader: transaction_capnp::chaum_pedersen_proof::Reader,
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

impl abcf::module::FromBytes for Transaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        let reader = read_message(bytes, ReaderOptions::new()).map_err(convert_capnp_error)?;

        let root = reader
            .get_root::<transaction_capnp::transaction::Reader>()
            .map_err(convert_capnp_error)?;

        let txid = root.get_txid().map_err(convert_capnp_error)?;

        let mut inputs = Vec::new();

        for input in root.get_inputs().map_err(convert_capnp_error)?.iter() {
            let txid = input.get_txid().map_err(convert_capnp_error)?.to_vec();
            let n = input.get_n();

            let operation = match input.get_operation().map_err(convert_capnp_noinschema)? {
                transaction_capnp::input::Operation::IssueAsset => InputOperation::IssueAsset,
                transaction_capnp::input::Operation::TransferAsset => InputOperation::TransferAsset,
            };

            let i = Input {
                txid,
                n,
                operation,
            };

            inputs.push(i);
        }

        let mut outputs = Vec::new();

        for output in root.get_outputs().map_err(convert_capnp_error)?.iter() {
            let public_key_bytes = output.get_public_key().map_err(convert_capnp_error)?;

            let public_key = ed25519_dalek::PublicKey::from_bytes(public_key_bytes)
                .map_err(|e| abcf::Error::ABCIApplicationError(90003, format!("{:?}", e)))?;

            let operation = match output.get_operation().map_err(convert_capnp_noinschema)? {
                transaction_capnp::output::Operation::IssueAsset => OutputOperation::IssueAsset,
                transaction_capnp::output::Operation::TransferAsset => {
                    OutputOperation::TransferAsset
                }
            };

            let amount = match output
                .get_amount()
                .which()
                .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?
            {
                transaction_capnp::output::amount::Which::Confidential(a) => {
                    let reader = a.map_err(convert_capnp_error)?;
                    let point0 = CompressedRistretto::zei_from_bytes(
                        reader.get_point0().map_err(convert_capnp_error)?,
                    )
                    .map_err(convert_ruc_error)?;
                    let point1 = CompressedRistretto::zei_from_bytes(
                        reader.get_point1().map_err(convert_capnp_error)?,
                    )
                    .map_err(convert_ruc_error)?;

                    XfrAmount::Confidential((point0, point1))
                }
                transaction_capnp::output::amount::Which::NonConfidential(a) => {
                    XfrAmount::NonConfidential(a)
                }
            };

            let asset_type = match output
                .get_asset()
                .which()
                .map_err(convert_capnp_noinschema)?
            {
                transaction_capnp::output::asset::Which::NonConfidential(a) => {
                    let point =
                        CompressedRistretto::zei_from_bytes(a.map_err(convert_capnp_error)?)
                            .map_err(convert_ruc_error)?;

                    XfrAssetType::Confidential(point)
                }
                transaction_capnp::output::asset::Which::Confidential(a) => {
                    let bytes: [u8; ASSET_TYPE_LENGTH] =
                        a.map_err(convert_capnp_error)?.try_into().map_err(|e| {
                            abcf::Error::ABCIApplicationError(90004, format!("{:?}", e))
                        })?;

                    let asset_type = AssetType(bytes);
                    XfrAssetType::NonConfidential(asset_type)
                }
            };

            let core = BlindAssetRecord {
                amount,
                asset_type,
                public_key: public_key.into(),
            };

            outputs.push(Output { core, operation })
        }

        let proof = {
            match root.get_proof().which().map_err(convert_capnp_noinschema)? {
                transaction_capnp::transaction::proof::Which::AssetMix(bytes) => {
                    let r1cs = bulletproofs::r1cs::R1CSProof::zei_from_bytes(
                        bytes.map_err(convert_capnp_error)?,
                    )
                    .map_err(convert_ruc_error)?;

                    AssetTypeAndAmountProof::AssetMix(r1cs.into())
                }
                transaction_capnp::transaction::proof::Which::ConfidentialAmount(e) => {
                    let reader = e.map_err(convert_capnp_error)?;

                    AssetTypeAndAmountProof::ConfAmount(parse_range_proof(reader)?)
                }
                transaction_capnp::transaction::proof::Which::ConfidentialAsset(e) => {
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
                            String::from(
                                "parse error, chaum_pedersen_proof_x must have 1 or 2 proof.",
                            ),
                        ));
                    };

                    AssetTypeAndAmountProof::ConfAsset(Box::new(proof))
                }
                transaction_capnp::transaction::proof::Which::ConfidentialAll(e) => {
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
                            String::from(
                                "parse error, chaum_pedersen_proof_x must have 1 or 2 proof.",
                            ),
                        ));
                    };

                    AssetTypeAndAmountProof::ConfAll(Box::new((range_proof, cpc_proof)))
                }
                transaction_capnp::transaction::proof::Which::NoProof(_) => {
                    AssetTypeAndAmountProof::NoProof
                }
            }
        };

        let mut signatures = Vec::new();

        for signature in root.get_signature().map_err(convert_capnp_error)?.iter() {
            let bytes = signature.map_err(convert_capnp_error)?;
            signatures.push(XfrSignature::zei_from_bytes(bytes).map_err(convert_ruc_error)?);
        }

        let tx = Transaction {
            txid: txid.to_vec(),
            inputs,
            outputs,
            proof,
            signatures
        };

        // validate tx.

        Ok(tx)
    }
}

impl Transaction {
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut result = Vec::new();

        let mut message = capnp::message::Builder::new_default();

        {
            let mut transaction = message.init_root::<transaction_capnp::transaction::Builder>();

            transaction.set_txid(&self.txid);

            // inputs
            let inputs_num: u32 = self.inputs.len().try_into().map_err(|e| eg!(format!("{}", e)))?;
            let mut inputs = transaction.reborrow().init_inputs(inputs_num);

            for i in 0 .. self.inputs.len() {
                let ori_input = &self.inputs[i];

                let index: u32 = i.try_into().map_err(|e| eg!(format!("{}", e)))?;

                let mut input = inputs.reborrow().get(index);

                input.set_txid(&ori_input.txid);
                input.set_n(ori_input.n);
                match ori_input.operation {
                    InputOperation::IssueAsset => input.set_operation(transaction_capnp::input::Operation::IssueAsset),
                    InputOperation::TransferAsset => input.set_operation(transaction_capnp::input::Operation::TransferAsset),
                }
            }

            // outputs
            let outputs_num: u32 = self.outputs.len().try_into().map_err(|e| eg!(format!("{}", e)))?;
            let mut outputs = transaction.reborrow().init_outputs(outputs_num);

            for i in 0 .. self.outputs.len() {
                let ori_output = &self.outputs[i];

                let index: u32 = i.try_into().map_err(|e| eg!(format!("{}", e)))?;

                let mut output = outputs.reborrow().get(index);

                let public_key = ori_output.core.public_key.zei_to_bytes();

                output.set_public_key(&public_key);

                match ori_output.operation {
                    OutputOperation::IssueAsset => output.set_operation(transaction_capnp::output::Operation::IssueAsset),
                    OutputOperation::TransferAsset => output.set_operation(transaction_capnp::output::Operation::TransferAsset),
                }

                let mut amount = output.reborrow().get_amount();

                match ori_output.core.amount {
                    XfrAmount::Confidential(e) => {
                        let point0 = e.0.zei_to_bytes();
                        let point1 = e.1.zei_to_bytes();

                        let mut c = amount.reborrow().init_confidential();

                        c.set_point0(&point0);
                        c.set_point1(&point1);

                    }
                    XfrAmount::NonConfidential(e) => {amount.set_non_confidential(e)}
                }

                let mut asset_type = output.reborrow().get_asset();

                match ori_output.core.asset_type {
                    XfrAssetType::NonConfidential(e) => {
                        let value = e.zei_to_bytes();
                        asset_type.set_non_confidential(&value);
                    },
                    XfrAssetType::Confidential(e) => {
                        let value = e.zei_to_bytes();
                        asset_type.set_confidential(&value);
                    }
                }
            }

            let signature_len: u32 = self.signatures.len().try_into().map_err(|e| eg!(format!("{}", e)))?;
            let mut signatures = transaction.reborrow().init_signature(signature_len);

            for i in 0 .. self.outputs.len() {
                let ori_sign = &self.signatures[i];

                let index: u32 = i.try_into().map_err(|e| eg!(format!("{}", e)))?;

                let value = ori_sign.zei_to_bytes();

                signatures.set(index, &value)
            }

            let mut proof = transaction.init_proof();

            match &self.proof {
                AssetTypeAndAmountProof::NoProof => { proof.reborrow().set_no_proof(()) },
                AssetTypeAndAmountProof::AssetMix(a) => {
                    // let value = a.
                }
            }
        }

        capnp::serialize_packed::write_message(&mut result, &message).c(d!())?;

        Ok(result)
    }
}

