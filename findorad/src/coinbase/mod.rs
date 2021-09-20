
use abcf::{Application, Event, bs3::model::{Map, Value}};
use serde::{Deserialize, Serialize};

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "coinbase", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct CoinbaseModule {
    // /// In memory.
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs]
impl CoinbaseModule {}

/// Module's block logic.
#[abcf::application]
impl Application for CoinbaseModule {
    type Transaction = MockTransaction;
}

/// Module's methods.
#[abcf::methods]
impl CoinbaseModule {}

pub struct MockTransaction {}

impl Default for MockTransaction {
    fn default() -> Self {
        MockTransaction {}
    }
}
