use std::convert::TryInto;

use capnp::{message::ReaderOptions, serialize::read_message};
use serde::{Deserialize, Serialize};
use zei::{
    ristretto::CompressedRistretto,
    xfr::{
        sig::XfrSignature,
        structs::{
            AssetType, AssetTypeAndAmountProof, BlindAssetRecord, XfrAmount, XfrAssetType,
            ASSET_TYPE_LENGTH,
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
}

impl abcf::Transaction for Transaction {}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            txid: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            proof: AssetTypeAndAmountProof::NoProof,
        }
    }
}

fn convert_capnp_error(e: capnp::Error) -> abcf::Error {
    abcf::Error::ABCIApplicationError(90001, format!("{:?}", e))
}

fn convert_capnp_noinschema(e: capnp::NotInSchema) -> abcf::Error {
    abcf::Error::ABCIApplicationError(90001, format!("{:?}", e))
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
            let signature_bytes = input.get_signature().map_err(convert_capnp_error)?;

            let signature: ed25519_dalek::Signature = signature_bytes
                .try_into()
                .map_err(|e| abcf::Error::ABCIApplicationError(90002, format!("{:?}", e)))?;

            let operation = match input.get_operation().map_err(convert_capnp_noinschema)? {
                transaction_capnp::input::Operation::IssueAsset => InputOperation::IssueAsset,
                transaction_capnp::input::Operation::TransferAsset => InputOperation::TransferAsset,
            };

            let i = Input {
                txid,
                n,
                operation,
                signature: XfrSignature(signature),
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
                    let point0_bytes = reader.get_point0().map_err(convert_capnp_error)?;
                    let point1_bytes = reader.get_point1().map_err(convert_capnp_error)?;

                    let point0 =
                        curve25519_dalek::ristretto::CompressedRistretto::from_slice(point0_bytes);
                    let point1 =
                        curve25519_dalek::ristretto::CompressedRistretto::from_slice(point1_bytes);

                    XfrAmount::Confidential((
                        CompressedRistretto(point0),
                        CompressedRistretto(point1),
                    ))
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
                    let point = curve25519_dalek::ristretto::CompressedRistretto::from_slice(
                        a.map_err(convert_capnp_error)?,
                    );

                    XfrAssetType::Confidential(CompressedRistretto(point))
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

        Ok(Self::default())
    }
}
