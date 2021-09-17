use crate::utxo::PublicKey;
use rand::Rng;
use rand_chacha::{rand_core, ChaChaRng};
use rand_core::{CryptoRng, RngCore, SeedableRng};
use serde::{Deserialize, Serialize};
use zei::xfr::structs::{AssetType as ZeiAssetType, ASSET_TYPE_LENGTH};

const DEFAULT_DECIMALS: u8 = 6;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetCode {
    val: ZeiAssetType,
}

impl Default for AssetCode {
    #[inline(always)]
    fn default() -> Self {
        Self {
            val: ZeiAssetType([255; ASSET_TYPE_LENGTH]),
        }
    }
}

impl AssetCode {
    /// Generate random asset code with custom engine
    #[inline(always)]
    pub fn gen_random_with_rng<R: RngCore + CryptoRng>(prng: &mut R) -> Self {
        let val: [u8; ASSET_TYPE_LENGTH] = prng.gen();
        Self {
            val: ZeiAssetType(val),
        }
    }

    /// Generate random asset type code with ChaChaRng
    #[inline(always)]
    pub fn gen_random() -> Self {
        Self::gen_random_with_rng(&mut ChaChaRng::from_entropy())
    }
}

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
struct AssetRules {
    pub(crate) transferable: bool,
    pub(crate) updatable: bool,
    // Optional limits on total issuance amount
    pub(crate) max_units: Option<u64>,
    pub(crate) decimals: u8,
}

impl Default for AssetRules {
    fn default() -> Self {
        Self {
            transferable: true,
            updatable: true,
            max_units: None,
            decimals: DEFAULT_DECIMALS,
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct AssetMemo {
    memo: String,
}

impl Default for AssetMemo {
    fn default() -> Self {
        Self {
            memo: "nonsense".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    code: AssetCode,
    units: u64,
    memo: AssetMemo,
    issuer: PublicKey,
    rules: AssetRules,
}

impl Asset {
    pub fn new(issuer: PublicKey) -> Self {
        let code = AssetCode::gen_random();
        let units = 0;
        let memo = Default::default();
        let rules = Default::default();

        Self {
            code,
            units,
            memo,
            issuer,
            rules,
        }
    }

    pub fn check_updatable(&self) -> bool {
        self.rules.updatable
    }

    pub fn check_transferable(&self) -> bool {
        self.rules.transferable
    }

    pub fn update_memo(&mut self, memo: AssetMemo) {
        self.memo = memo;
    }

    pub fn units(&self) -> u64 {
        self.units
    }

    pub fn issue(&mut self, units: u64) {
        self.units = units;
    }
}
