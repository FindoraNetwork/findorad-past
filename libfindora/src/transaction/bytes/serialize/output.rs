use crate::{
    staking::ValidatorPublicKey,
    transaction::{Output, OutputOperation},
    transaction_capnp::output,
    Result,
};
use zei::{
    serialization::ZeiFromToBytes,
    xfr::structs::{XfrAmount, XfrAssetType},
};

use super::evm::build_evm;

pub fn build_output(output: &Output, builder: output::Builder) -> Result<()> {
    let mut builder = builder;

    {
        let mut builder = builder.reborrow().init_address();
        builder.set_address(output.core.address.as_ref())
    }

    {
        let mut builder = builder.reborrow().init_asset();
        match &output.core.asset {
            XfrAssetType::Confidential(a) => builder.set_confidential(a.0.as_bytes()),
            XfrAssetType::NonConfidential(a) => builder.set_non_confidential(a.0.as_ref()),
        }
    }

    {
        let mut builder = builder.reborrow().init_amount();
        match &output.core.amount {
            XfrAmount::NonConfidential(a) => builder.set_non_confidential(*a),
            XfrAmount::Confidential(a) => {
                let mut builder = builder.init_confidential();

                builder.set_point0(a.0.zei_to_bytes().as_ref());
                builder.set_point1(a.1.zei_to_bytes().as_ref());
            }
        }
    }

    {
        let mut builder = builder.reborrow().init_owner_memo();
        match &output.core.owner_memo {
            Some(a) => {
                let mut omb = builder.init_some();
                omb.set_blind_share(&a.blind_share.zei_to_bytes());
                omb.set_ctext(&a.lock.ciphertext.zei_to_bytes());
                omb.set_ephemeral_public_key(&a.lock.ephemeral_public_key.zei_to_bytes());
            }
            None => builder.set_none(()),
        }
    }

    {
        let mut operation = builder.init_operation();
        match &output.operation {
            OutputOperation::DefineAsset(a) => {
                let mut asset_meta = operation.init_define_asset();
                asset_meta.set_transferable(a.transferable);

                let mut maximum = asset_meta.init_maximum();

                match a.maximum {
                    Some(v) => {
                        let mut bytes = [0u8; 32];
                        v.to_big_endian(&mut bytes);
                        maximum.set_some(&bytes);
                    }
                    None => maximum.set_none(()),
                }
            }
            OutputOperation::IssueAsset => operation.set_issue_asset(()),
            OutputOperation::TransferAsset => operation.set_transfer_asset(()),
            OutputOperation::Fee => operation.set_fee(()),
            OutputOperation::Undelegate(a) => {
                let mut undelegate = operation.init_undelegate();
                undelegate.reborrow().set_address(a.address.0.as_ref());
            }
            OutputOperation::Delegate(a) => {
                let mut delegation = operation.init_delegate();
                delegation.reborrow().set_address(a.address.0.as_ref());

                let mut validator = delegation.reborrow().init_validator();

                match &a.validator {
                    Some(v) => {
                        let mut key = validator.reborrow().init_some();
                        let mut k = key.reborrow().init_key();
                        match v {
                            ValidatorPublicKey::Ed25519(v) => {
                                k.set_ed25519(v);
                            }
                            ValidatorPublicKey::Secp256k1(v) => {
                                k.set_secp256k1(v);
                            }
                            ValidatorPublicKey::Unknown => k.set_unknown(()),
                        }
                    }
                    None => validator.set_none(()),
                }

                let mut memo = delegation.init_memo();

                match &a.memo {
                    Some(v) => memo.set_some(v.as_ref()),
                    None => memo.set_nono(()),
                }
            }
            OutputOperation::ClaimReward(a) => {
                let mut claim = operation.init_claim_reward();
                claim.set_validator(a.validator.0.as_ref());
            }
            OutputOperation::EvmCall(a) => {
                let builder = operation.init_evm_call();
                build_evm(a, builder)?;
            }
        }
    }

    Ok(())
}
