use zei::{serialization::ZeiFromToBytes, xfr::structs::AssetTypeAndAmountProof};

use crate::transaction_capnp::transaction::proof;

pub fn build_proof(proof: &AssetTypeAndAmountProof, builder: proof::Builder) -> abcf::Result<()> {
    let mut builder = builder;

    match proof {
        AssetTypeAndAmountProof::NoProof => builder.reborrow().set_no_proof(()),
        AssetTypeAndAmountProof::AssetMix(a) => {
            let value = a.into_r1cs().zei_to_bytes();
            builder.reborrow().set_asset_mix(&value);
        }
        AssetTypeAndAmountProof::ConfAsset(a) => {
            let len = if a.zero.is_some() { 2 } else { 1 };

            let mut ca = builder.reborrow().init_confidential_asset(len);

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

            let mut ca = builder.reborrow().init_confidential_amount();
            ca.set_range_proof(&range_proof);
            ca.set_diff_commitment_low(&low);
            ca.set_diff_commitment_high(&high);
        }
        AssetTypeAndAmountProof::ConfAll(a) => {
            let mut proof = builder.init_confidential_all();
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
    Ok(())
}
