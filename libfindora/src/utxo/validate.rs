use rand_core::{CryptoRng, RngCore};
use ruc::*;
use zei::{
    setup::PublicParams,
    xfr::{
        lib::verify_bare_transaction,
        structs::{AssetTypeAndAmountProof, BlindAssetRecord},
    },
};

pub struct ValidateTransaction {
    pub inputs: Vec<BlindAssetRecord>,
    pub outputs: Vec<BlindAssetRecord>,
    pub proof: AssetTypeAndAmountProof,
}

impl ValidateTransaction {
    pub fn verify<C: CryptoRng + RngCore>(
        &self,
        prng: &mut C,
        params: &mut PublicParams,
    ) -> Result<()> {
        verify_bare_transaction(
            prng,
            params,
            self.inputs.as_ref(),
            self.outputs.as_ref(),
            &self.proof,
        )
    }
}
