use abcf::tm_protos::crypto;
use primitive_types::H160;

use crate::{
    error::{
        convert_capnp_error, convert_capnp_noinschema, convert_ruc_error, convert_try_slice_error,
    },
    rewards,
    staking::{self, TendermintAddress},
    transaction::{Output, OutputOperation},
    transaction_capnp::output,
    utxo::{self, Address, FraAddress},
};
use zei::{
    hybrid_encryption::{XPublicKey, ZeiHybridCipher},
    ristretto::{CompressedEdwardsY, CompressedRistretto},
    serialization::ZeiFromToBytes,
    xfr::{
        sig::XfrPublicKey,
        structs::{AssetType, OwnerMemo, XfrAmount, XfrAssetType, ASSET_TYPE_LENGTH},
    },
};

pub fn from_output(reader: output::Reader) -> abcf::Result<Output> {
    let address = from_address(reader.get_address())?;
    let amount = from_amount(reader.get_amount())?;
    let asset = from_asset(reader.get_asset())?;
    let operation = from_operation(reader.get_operation())?;
    let owner_memo = from_owner_memo(reader.get_owner_memo())?;

    let core = utxo::Output {
        address,
        amount,
        asset,
        owner_memo,
    };

    Ok(Output { core, operation })
}

fn from_owner_memo(reader: output::owner_memo::Reader) -> abcf::Result<Option<OwnerMemo>> {
    let owner_memo = match reader.which().map_err(convert_capnp_noinschema)? {
        output::owner_memo::None(_) => None,
        output::owner_memo::Some(a) => {
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
    Ok(owner_memo)
}

fn from_operation(reader: output::operation::Reader) -> abcf::Result<OutputOperation> {
    use crate::transaction_capnp::delegate_data::memo;
    use crate::transaction_capnp::delegate_data::validator;
    use crate::transaction_capnp::validator_key;
    use output::operation;

    let operation = match reader
        .which()
        .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?
    {
        operation::Which::IssueAsset(_) => OutputOperation::IssueAsset,
        operation::Which::TransferAsset(_) => OutputOperation::TransferAsset,
        operation::Which::Fee(_) => OutputOperation::Fee,
        operation::Which::Delegate(a) => {
            let reader = a.map_err(convert_capnp_error)?;
            let validator = match reader
                .get_validator()
                .which()
                .map_err(convert_capnp_noinschema)?
            {
                validator::Which::None(_) => None,
                validator::Which::Some(b) => {
                    let b_reader = b.map_err(convert_capnp_error)?;
                    let key = match b_reader
                        .get_key()
                        .which()
                        .map_err(convert_capnp_noinschema)?
                    {
                        validator_key::key::Which::Ed25519(a) => crypto::public_key::Sum::Ed25519(
                            a.map_err(convert_capnp_error)?.to_vec(),
                        ),
                        validator_key::key::Which::Secp256k1(a) => {
                            crypto::public_key::Sum::Secp256k1(
                                a.map_err(convert_capnp_error)?.to_vec(),
                            )
                        }
                    };
                    Some(crypto::PublicKey { sum: Some(key) })
                }
            };

            let address = reader.get_address().map_err(convert_capnp_error)?;

            let td_address =
                TendermintAddress(address.try_into().map_err(convert_try_slice_error)?);

            let memo = match reader
                .get_memo()
                .which()
                .map_err(convert_capnp_noinschema)?
            {
                memo::Which::Some(d) => Some(d.map_err(convert_capnp_error)?.to_vec()),
                memo::Which::Nono(_) => None,
            };

            OutputOperation::Delegate(staking::Delegate {
                address: td_address,
                validator,
                memo: memo,
            })
        }
        operation::Which::Undelegate(a) => {
            let reader = a.map_err(convert_capnp_error)?;
            let address = reader.get_address().map_err(convert_capnp_error)?;
            let td_address =
                TendermintAddress(address.try_into().map_err(convert_try_slice_error)?);

            OutputOperation::Undelegate(staking::Undelegate {
                address: td_address,
            })
        }
        operation::Which::ClaimReward(a) => {
            let reader = a.map_err(convert_capnp_error)?;
            let address = reader.get_validator().map_err(convert_capnp_error)?;
            let td_address =
                TendermintAddress(address.try_into().map_err(convert_try_slice_error)?);

            OutputOperation::ClaimReward(rewards::Claim {
                validator: td_address,
            })
        }
    };
    Ok(operation)
}

fn from_asset(reader: output::asset::Reader) -> abcf::Result<XfrAssetType> {
    let asset_type = match reader.which().map_err(convert_capnp_noinschema)? {
        output::asset::Which::Confidential(a) => {
            let point = CompressedRistretto::zei_from_bytes(a.map_err(convert_capnp_error)?)
                .map_err(convert_ruc_error)?;

            XfrAssetType::Confidential(point)
        }
        output::asset::Which::NonConfidential(a) => {
            let bytes: [u8; ASSET_TYPE_LENGTH] = a
                .map_err(convert_capnp_error)?
                .try_into()
                .map_err(|e| abcf::Error::ABCIApplicationError(90004, format!("{:?}", e)))?;

            let asset_type = AssetType(bytes);
            XfrAssetType::NonConfidential(asset_type)
        }
    };
    Ok(asset_type)
}

fn from_amount(reader: output::amount::Reader) -> abcf::Result<XfrAmount> {
    let amount = match reader
        .which()
        .map_err(|e| abcf::Error::ABCIApplicationError(90001, format!("{:?}", e)))?
    {
        output::amount::Which::Confidential(a) => {
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
        output::amount::Which::NonConfidential(a) => XfrAmount::NonConfidential(a),
    };
    Ok(amount)
}

fn from_address(reader: output::address::Reader) -> abcf::Result<Address> {
    Ok(match reader.which().map_err(convert_capnp_noinschema)? {
        output::address::Eth(a) => {
            let reader = a.map_err(convert_capnp_error)?;
            let inner = reader.try_into().map_err(convert_try_slice_error)?;
            Address::Eth(H160(inner))
        }
        output::address::Fra(a) => {
            let reader = a.map_err(convert_capnp_error)?;
            let address_reader = reader.get_address().map_err(convert_capnp_error)?;
            let inner = address_reader.try_into().map_err(convert_try_slice_error)?;
            let address = H160(inner);

            let public_key_reader = reader.get_public_key().map_err(convert_capnp_error)?;
            let public_key =
                XfrPublicKey::zei_from_bytes(public_key_reader).map_err(convert_ruc_error)?;
            Address::Fra(FraAddress {
                address,
                public_key,
            })
        }
    })
}
