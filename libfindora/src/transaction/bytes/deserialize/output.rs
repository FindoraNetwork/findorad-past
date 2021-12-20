use primitive_types::{H160, U256};

use crate::{
    asset::AssetMeta,
    rewards,
    staking::{self, TendermintAddress, ValidatorPublicKey},
    transaction::{Output, OutputOperation, bytes::deserialize::evm::from_evm},
    transaction_capnp::{address, output},
    utxo, Address, Result,
};
use zei::{
    hybrid_encryption::{XPublicKey, ZeiHybridCipher},
    ristretto::{CompressedEdwardsY, CompressedRistretto},
    serialization::ZeiFromToBytes,
    xfr::structs::{AssetType, OwnerMemo, XfrAmount, XfrAssetType, ASSET_TYPE_LENGTH},
};

pub fn from_output(reader: output::Reader) -> Result<Output> {
    let address = from_address(reader.get_address()?)?;
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

fn from_owner_memo(reader: output::owner_memo::Reader) -> Result<Option<OwnerMemo>> {
    let owner_memo = match reader.which()? {
        output::owner_memo::None(_) => None,
        output::owner_memo::Some(a) => {
            // None
            let reader = a?;

            let ctext = zei::hybrid_encryption::Ctext::zei_from_bytes(reader.get_ctext()?)?;
            let ephemeral_public_key =
                XPublicKey::zei_from_bytes(reader.get_ephemeral_public_key()?)?;
            let cipher = ZeiHybridCipher {
                ciphertext: ctext,
                ephemeral_public_key,
            };

            let blind_share = CompressedEdwardsY::zei_from_bytes(reader.get_blind_share()?)?;

            Some(OwnerMemo {
                blind_share,
                lock: cipher,
            })
        }
    };
    Ok(owner_memo)
}

fn from_operation(reader: output::operation::Reader) -> Result<OutputOperation> {
    use crate::transaction_capnp::define_asset;
    use crate::transaction_capnp::delegate_data::memo;
    use crate::transaction_capnp::delegate_data::validator;
    use crate::transaction_capnp::validator_key;
    use output::operation;

    let operation = match reader.which()? {
        operation::Which::DefineAsset(e) => {
            let reader = e?;

            let transferable = reader.get_transferable();

            let maximum = match reader.get_maximum().which()? {
                define_asset::maximum::Which::None(_) => None,
                define_asset::maximum::Which::Some(a) => {
                    let reader = a?;
                    Some(U256::from_big_endian(reader))
                }
            };

            OutputOperation::DefineAsset(AssetMeta {
                transferable,
                maximum,
            })
        }
        operation::Which::IssueAsset(_) => OutputOperation::IssueAsset,
        operation::Which::TransferAsset(_) => OutputOperation::TransferAsset,
        operation::Which::Fee(_) => OutputOperation::Fee,
        operation::Which::Delegate(a) => {
            let reader = a?;
            let validator = match reader.get_validator().which()? {
                validator::Which::None(_) => None,
                validator::Which::Some(b) => {
                    let b_reader = b?;
                    let key = match b_reader.get_key().which()? {
                        validator_key::key::Which::Ed25519(a) => {
                            ValidatorPublicKey::Ed25519(a?.to_vec())
                        }
                        validator_key::key::Which::Secp256k1(a) => {
                            ValidatorPublicKey::Secp256k1(a?.to_vec())
                        }
                        validator_key::key::Which::Unknown(_) => ValidatorPublicKey::Unknown,
                    };
                    Some(key)
                }
            };

            let address = reader.get_address()?;

            let td_address = TendermintAddress(address.try_into()?);

            let memo = match reader.get_memo().which()? {
                memo::Which::Some(d) => Some(d?.to_vec()),
                memo::Which::Nono(_) => None,
            };

            OutputOperation::Delegate(staking::Delegate {
                address: td_address,
                validator,
                memo,
            })
        }
        operation::Which::Undelegate(a) => {
            let reader = a?;
            let address = reader.get_address()?;
            let td_address = TendermintAddress(address.try_into()?);

            OutputOperation::Undelegate(staking::Undelegate {
                address: td_address,
            })
        }
        operation::Which::ClaimReward(a) => {
            let reader = a?;
            let address = reader.get_validator()?;
            let td_address = TendermintAddress(address.try_into()?);

            OutputOperation::ClaimReward(rewards::Claim {
                validator: td_address,
            })
        }
        operation::Which::EvmCall(a) => {
            let reader = a?;

            OutputOperation::EvmCall(from_evm(reader)?)
        }
    };
    Ok(operation)
}

fn from_asset(reader: output::asset::Reader) -> Result<XfrAssetType> {
    let asset_type = match reader.which()? {
        output::asset::Which::Confidential(a) => {
            let point = CompressedRistretto::zei_from_bytes(a?)?;

            XfrAssetType::Confidential(point)
        }
        output::asset::Which::NonConfidential(a) => {
            let bytes: [u8; ASSET_TYPE_LENGTH] = a?.try_into()?;

            let asset_type = AssetType(bytes);
            XfrAssetType::NonConfidential(asset_type)
        }
    };
    Ok(asset_type)
}

fn from_amount(reader: output::amount::Reader) -> Result<XfrAmount> {
    let amount = match reader.which()? {
        output::amount::Which::Confidential(a) => {
            let reader = a?;
            let point0 = CompressedRistretto::zei_from_bytes(reader.get_point0()?)?;
            let point1 = CompressedRistretto::zei_from_bytes(reader.get_point1()?)?;

            XfrAmount::Confidential((point0, point1))
        }
        output::amount::Which::NonConfidential(a) => XfrAmount::NonConfidential(a),
    };
    Ok(amount)
}

pub fn from_address(reader: address::Reader) -> Result<Address> {
    Ok(match reader.which()? {
        address::Which::Eth(a) => {
            let reader = a?;
            let inner = reader.try_into()?;
            Address::Eth(H160(inner))
        }
        address::Which::Fra(a) => {
            let reader = a?;
            let inner = reader.try_into()?;
            Address::Fra(H160(inner))
        }
        address::Which::BlockHole(_) => Address::BlockHole,
    })
}
