use crate::{
    transaction::{Output, OutputOperation},
    transaction_capnp::output,
    utxo::Address,
};
use abcf::tm_protos::crypto;
use zei::{
    serialization::ZeiFromToBytes,
    xfr::structs::{XfrAmount, XfrAssetType},
};

pub fn build_output(output: &Output, builder: output::Builder) -> abcf::Result<()> {
    let mut builder = builder;

    {
        let mut builder = builder.reborrow().init_address();
        match &output.core.address {
            Address::Eth(a) => builder.set_eth(a.0.as_ref()),
            Address::Fra(a) => builder.set_fra(a.0.as_ref()),
        }
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
            OutputOperation::IssueAsset => operation.set_issue_asset(()),
            OutputOperation::TransferAsset => operation.set_transfer_asset(()),
            OutputOperation::Fee => operation.set_fee(()),
            OutputOperation::Undelegate(a) => {
                let mut undelegate = operation.init_undelegate();
                undelegate.reborrow().set_address(&a.address.0.as_ref());
            }
            OutputOperation::Delegate(a) => {
                let mut delegation = operation.init_delegate();
                delegation.reborrow().set_address(&a.address.0.as_ref());

                let mut validator = delegation.reborrow().init_validator();

                match &a.validator {
                    Some(v) => {
                        let mut key = validator.reborrow().init_some();
                        let mut k = key.reborrow().init_key();
                        match &v.sum {
                            Some(v) => match v {
                                crypto::public_key::Sum::Ed25519(v) => {
                                    k.set_ed25519(v.as_ref());
                                }
                                crypto::public_key::Sum::Secp256k1(v) => {
                                    k.set_secp256k1(v.as_ref());
                                }
                            },
                            None => validator.set_none(()),
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
        }
    }

    Ok(())
}
