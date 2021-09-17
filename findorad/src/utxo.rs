use abcf_utxo::{UTXOModule, Config};

#[derive(Debug, Clone)]
pub struct UTXOConfig {}

// TODO: use zei type
impl Config for UTXOConfig {
    type Address = u64;

    type Signature = u64;

    type AssetCode = u64;

    type PublicKey = u64;

    type OutputId = u64;
}

pub type UTXO<S> = UTXOModule<UTXOConfig, S>;
pub type UtxoTx = abcf_utxo::Transaction<UTXOConfig>;

