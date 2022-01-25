use crate::evm::EvmMemo;

#[derive(Debug)]
pub enum Memo {
    Ethereum(EvmMemo),
}
