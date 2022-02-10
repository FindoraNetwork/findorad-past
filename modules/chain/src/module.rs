use abcf::{
    bs3::{
        merkle::empty::EmptyMerkle,
        model::{Map, Value},
    },
    Application,
};

use crate::Transaction;

#[abcf::module(name = "chain", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct ChainModule {
    #[stateful(merkle = "EmptyMerkle")]
    pub sf_value: Value<u32>,
    // Only a placeholder, will remove when abcf update.
    #[stateless]
    pub heights: Map<i64, [u8; 32]>,
    #[stateless]
    pub hashs: Map<[u8; 32], i64>,
}

#[abcf::rpcs]
impl ChainModule {}

/// Module's block logic.
#[abcf::application]
impl Application for ChainModule {
    type Transaction = Transaction;
}

/// Module's methods.
#[abcf::methods]
impl ChainModule {}
