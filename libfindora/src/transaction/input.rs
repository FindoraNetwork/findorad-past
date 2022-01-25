use primitive_types::H512;

use crate::evm;

#[derive(Debug, Clone)]
pub enum Operation {
    TransferAsset,
    EvmCall(evm::Input),
}

#[derive(Debug, Clone)]
pub struct Input {
    pub txid: H512,
    pub n: u32,
    pub operation: Operation,
}
