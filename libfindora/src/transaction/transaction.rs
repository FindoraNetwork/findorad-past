use std::convert::TryInto;

use abcf::ToBytes;
use capnp::{message::ReaderOptions, serialize_packed};
use digest::Digest;
use ruc::*;
use serde::{Deserialize, Serialize};
use sha3::Sha3_512;
use zei::{
    chaum_pedersen::{ChaumPedersenProof, ChaumPedersenProofX},
    hybrid_encryption::{XPublicKey, ZeiHybridCipher},
    ristretto::{CompressedEdwardsY, CompressedRistretto, RistrettoPoint, RistrettoScalar},
    serialization::ZeiFromToBytes,
    xfr::{
        sig::{XfrKeyPair, XfrSignature},
        structs::{
            AssetType, AssetTypeAndAmountProof, BlindAssetRecord, OwnerMemo, XfrAmount,
            XfrAssetType, XfrRangeProof, ASSET_TYPE_LENGTH,
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
        let reader = serialize_packed::read_message(bytes, ReaderOptions::new()).map_err(convert_capnp_error)?;
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
                transaction_capnp::input::Operation::Undelegate => InputOperation::Undelegate,
                transaction_capnp::input::Operation::ClaimReward => InputOperation::ClaimReward,
            };

            let i = Input { txid, n, operation };

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
                transaction_capnp::output::Operation::Fee => OutputOperation::Fee,
                transaction_capnp::output::Operation::Undelegate => OutputOperation::Undelegate,
                transaction_capnp::output::Operation::Delegate => OutputOperation::Delegate,
                transaction_capnp::output::Operation::ClaimReward => OutputOperation::ClaimReward,
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
                transaction_capnp::output::asset::Which::Confidential(a) => {
                    let point =
                        CompressedRistretto::zei_from_bytes(a.map_err(convert_capnp_error)?)
                            .map_err(convert_ruc_error)?;

                    XfrAssetType::Confidential(point)
                }
                transaction_capnp::output::asset::Which::NonConfidential(a) => {
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

            let owner_memo = match output
                .get_owner_memo()
                .which()
                .map_err(convert_capnp_noinschema)?
            {
                transaction_capnp::output::owner_memo::None(_) => None,
                transaction_capnp::output::owner_memo::Some(a) => {
                    // None
                    let reader = a.map_err(convert_capnp_error)?;

                    let ctext = zei::hybrid_encryption::Ctext::zei_from_bytes(
                        reader.get_ctext().map_err(convert_capnp_error)?,
                    )
                    .map_err(convert_ruc_error)?;
                    let ephemeral_public_key = XPublicKey::zei_from_bytes(
                        reader
                            .get_ephemeral_public_key()
                            .map_err(convert_capnp_error)?,
                    )
                    .map_err(convert_ruc_error)?;
                    let cipher = ZeiHybridCipher {
                        ciphertext: ctext,
                        ephemeral_public_key,
                    };

                    let blind_share = CompressedEdwardsY::zei_from_bytes(
                        reader.get_blind_share().map_err(convert_capnp_error)?,
                    )
                    .map_err(convert_ruc_error)?;

                    Some(OwnerMemo {
                        blind_share,
                        lock: cipher,
                    })
                }
            };

            outputs.push(Output {
                core,
                operation,
                owner_memo: owner_memo,
            })
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
            signatures,
        };

        // validate tx.

        Ok(tx)
    }
}

impl Transaction {
    pub fn signature(&mut self, keypairs: Vec<XfrKeyPair>) -> Result<()> {
        if self.signatures.len() != 0 {
            return Err(eg!("this tx is signed."));
        }

        if self.inputs.len() != keypairs.len() {
            return Err(eg!("please give right keypair for inputs."));
        }

        let bytes = self.to_bytes().map_err(|e| eg!(format!("{:?}", e)))?;

        for i in 0..keypairs.len() {
            let keypair = &keypairs[i];

            let signature = keypair.sign(&bytes);

            self.signatures.push(signature);
        }

        let bytes = self.to_bytes().map_err(|e| eg!(format!("{:?}", e)))?;

        self.txid = Sha3_512::digest(&bytes).to_vec();

        Ok(())
    }
}

impl ToBytes for Transaction {
    fn to_bytes(&self) -> abcf::Result<Vec<u8>> {
        let mut result = Vec::new();

        let mut message = capnp::message::Builder::new_default();

        {
            let mut transaction = message.init_root::<transaction_capnp::transaction::Builder>();

            transaction.set_txid(&self.txid);

            // inputs
            let inputs_num: u32 = self
                .inputs
                .len()
                .try_into()
                .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

            let mut inputs = transaction.reborrow().init_inputs(inputs_num);

            for i in 0..self.inputs.len() {
                let ori_input = &self.inputs[i];

                let index: u32 = i
                    .try_into()
                    .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

                let mut input = inputs.reborrow().get(index);

                input.set_txid(&ori_input.txid);
                input.set_n(ori_input.n);
                match ori_input.operation {
                    InputOperation::IssueAsset => {
                        input.set_operation(transaction_capnp::input::Operation::IssueAsset)
                    }
                    InputOperation::TransferAsset => {
                        input.set_operation(transaction_capnp::input::Operation::TransferAsset)
                    }
                    InputOperation::Undelegate => {
                        input.set_operation(transaction_capnp::input::Operation::Undelegate)
                    }
                    InputOperation::ClaimReward => {
                        input.set_operation(transaction_capnp::input::Operation::ClaimReward)
                    }
                }
            }

            // outputs
            let outputs_num: u32 = self
                .outputs
                .len()
                .try_into()
                .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

            let mut outputs = transaction.reborrow().init_outputs(outputs_num);

            for i in 0..self.outputs.len() {
                let ori_output = &self.outputs[i];

                let index: u32 = i
                    .try_into()
                    .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

                let mut output = outputs.reborrow().get(index);

                let public_key = ori_output.core.public_key.zei_to_bytes();

                output.set_public_key(&public_key);

                match ori_output.operation {
                    OutputOperation::IssueAsset => {
                        output.set_operation(transaction_capnp::output::Operation::IssueAsset)
                    }
                    OutputOperation::TransferAsset => {
                        output.set_operation(transaction_capnp::output::Operation::TransferAsset)
                    }
                    OutputOperation::Fee => {
                        output.set_operation(transaction_capnp::output::Operation::Fee)
                    }
                    OutputOperation::Undelegate => {
                        output.set_operation(transaction_capnp::output::Operation::Undelegate)
                    },
                    OutputOperation::Delegate => {
                        output.set_operation(transaction_capnp::output::Operation::Delegate)
                    }
                    OutputOperation::ClaimReward => {
                        output.set_operation(transaction_capnp::output::Operation::ClaimReward)
                    }
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
                    XfrAmount::NonConfidential(e) => amount.set_non_confidential(e),
                }

                let mut asset_type = output.reborrow().get_asset();

                match ori_output.core.asset_type {
                    XfrAssetType::NonConfidential(e) => {
                        let value = e.zei_to_bytes();
                        asset_type.set_non_confidential(&value);
                    }
                    XfrAssetType::Confidential(e) => {
                        let value = e.zei_to_bytes();
                        asset_type.set_confidential(&value);
                    }
                }

                let mut owner_memo = output.reborrow().get_owner_memo();

                match &ori_output.owner_memo {
                    Some(om) => {
                        let mut omb = owner_memo.init_some();
                        omb.set_blind_share(&om.blind_share.zei_to_bytes());
                        omb.set_ctext(&om.lock.ciphertext.zei_to_bytes());
                        omb.set_ephemeral_public_key(&om.lock.ephemeral_public_key.zei_to_bytes());
                    }
                    None => owner_memo.set_none(()),
                }
            }

            let signature_len: u32 = self
                .signatures
                .len()
                .try_into()
                .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?;

            let mut signatures = transaction.reborrow().init_signature(signature_len);

            for i in 0..self.inputs.len() {
                if let Some(signature) = self.signatures.get(i) {
                    let ori_sign = signature;

                    let index: u32 = i.try_into().map_err(|e| {
                        abcf::Error::ABCIApplicationError(90001, format!("{:?}", e))
                    })?;

                    let value = ori_sign.zei_to_bytes();

                    signatures.set(index, &value)
                }
            }

            let mut proof = transaction.init_proof();

            match &self.proof {
                AssetTypeAndAmountProof::NoProof => proof.reborrow().set_no_proof(()),
                AssetTypeAndAmountProof::AssetMix(a) => {
                    let value = a.into_r1cs().zei_to_bytes();
                    proof.reborrow().set_asset_mix(&value);
                }
                AssetTypeAndAmountProof::ConfAsset(a) => {
                    let len = if a.zero.is_some() { 2 } else { 1 };

                    let mut ca = proof.reborrow().init_confidential_asset(len);

                    {
                        let mut p1 = ca.reborrow().get(0);

                        let c3 = a.c1_eq_c2.c3.zei_to_bytes();
                        let c4 = a.c1_eq_c2.c4.zei_to_bytes();
                        let z1 = a.c1_eq_c2.z1.zei_to_bytes();
                        let z2 = a.c1_eq_c2.z2.zei_to_bytes();
                        let z3 = a.c1_eq_c2.z3.zei_to_bytes();

                        p1.set_c3(&c3);
                        p1.set_c4(&c4);
                        p1.set_z1(&z1);
                        p1.set_z2(&z2);
                        p1.set_z3(&z3);
                    }

                    if let Some(e) = &a.zero {
                        let mut p1 = ca.reborrow().get(1);

                        let c3 = e.c3.zei_to_bytes();
                        let c4 = e.c4.zei_to_bytes();
                        let z1 = e.z1.zei_to_bytes();
                        let z2 = e.z2.zei_to_bytes();
                        let z3 = e.z3.zei_to_bytes();

                        p1.set_c3(&c3);
                        p1.set_c4(&c4);
                        p1.set_z1(&z1);
                        p1.set_z2(&z2);
                        p1.set_z3(&z3);
                    }
                }
                AssetTypeAndAmountProof::ConfAmount(a) => {
                    let range_proof = a.range_proof.zei_to_bytes();
                    let low = a.xfr_diff_commitment_low.zei_to_bytes();
                    let high = a.xfr_diff_commitment_high.zei_to_bytes();

                    let mut ca = proof.reborrow().init_confidential_amount();
                    ca.set_range_proof(&range_proof);
                    ca.set_diff_commitment_low(&low);
                    ca.set_diff_commitment_high(&high);
                }
                AssetTypeAndAmountProof::ConfAll(a) => {
                    let mut proof = proof.init_confidential_all();
                    {
                        let r = &a.0;

                        let mut ca = proof.reborrow().init_amount();
                        let range_proof = r.range_proof.zei_to_bytes();
                        let low = r.xfr_diff_commitment_low.zei_to_bytes();
                        let high = r.xfr_diff_commitment_high.zei_to_bytes();

                        ca.set_range_proof(&range_proof);
                        ca.set_diff_commitment_low(&low);
                        ca.set_diff_commitment_high(&high);
                    }
                    {
                        let p = &a.1;

                        let len = if p.zero.is_some() { 2 } else { 1 };

                        let mut ca = proof.init_asset(len);
                        {
                            let mut p1 = ca.reborrow().get(0);

                            let c3 = p.c1_eq_c2.c3.zei_to_bytes();
                            let c4 = p.c1_eq_c2.c4.zei_to_bytes();
                            let z1 = p.c1_eq_c2.z1.zei_to_bytes();
                            let z2 = p.c1_eq_c2.z2.zei_to_bytes();
                            let z3 = p.c1_eq_c2.z3.zei_to_bytes();

                            p1.set_c3(&c3);
                            p1.set_c4(&c4);
                            p1.set_z1(&z1);
                            p1.set_z2(&z2);
                            p1.set_z3(&z3);
                        }

                        if let Some(e) = &p.zero {
                            let mut p1 = ca.reborrow().get(1);

                            let c3 = e.c3.zei_to_bytes();
                            let c4 = e.c4.zei_to_bytes();
                            let z1 = e.z1.zei_to_bytes();
                            let z2 = e.z2.zei_to_bytes();
                            let z3 = e.z3.zei_to_bytes();

                            p1.set_c3(&c3);
                            p1.set_c4(&c4);
                            p1.set_z1(&z1);
                            p1.set_z2(&z2);
                            p1.set_z3(&z3);
                        }
                    }
                }
            }
        }

        serialize_packed::write_message(&mut result, &message)
            .map_err(|e| abcf::Error::ABCIApplicationError(90002, format!("{:?}", e)))?;
        Ok(result)
    }
}
