use abcf::bs3::{DoubleKeyMapStore, MapStore};
use evm::backend::Basic;
use fm_utxo::Result;
use libfindora::{
    asset::FRA,
    utxo::{Output, OutputId},
    Address,
};
use primitive_types::{H160, H256, U256};

use super::{account::Account, vicinity::Vicinity};

pub struct Backend<'a, OS, OO, A, S> {
    pub vicinity: &'a Vicinity,
    pub owned_outputs: OO,
    pub outputs_sets: OS,
    pub accounts: A,
    pub storages: S,
}

impl<'a, OS, OO, A, S> Backend<'a, OS, OO, A, S>
where
    OS: MapStore<OutputId, Output>,
    OO: MapStore<Address, Vec<OutputId>>,
    A: MapStore<H160, Account>,
    S: DoubleKeyMapStore<H160, H256, H256>,
{
    fn _basic(&self, address: H160) -> Result<Basic> {
        let balances = fm_utxo::utils::balance(
            Address::from(address),
            &self.outputs_sets,
            &self.owned_outputs,
        )?;

        let balance = if let Some(i) = balances.get(&FRA.bare_asset_type) {
            *i
        } else {
            0
        };

        let nonce = if let Some(a) = self.accounts.get(&address)? {
            a.nonce
        } else {
            0
        };

        let basic = Basic {
            balance: U256::from(balance),
            nonce: U256::from(nonce),
        };

        Ok(basic)
    }
}

impl<'a, OS, OO, A, S> evm::backend::Backend for Backend<'a, OS, OO, A, S>
where
    OS: MapStore<OutputId, Output>,
    OO: MapStore<Address, Vec<OutputId>>,
    A: MapStore<H160, Account>,
    S: DoubleKeyMapStore<H160, H256, H256>,
{
    fn gas_price(&self) -> U256 {
        self.vicinity.gas_price
    }
    fn origin(&self) -> H160 {
        self.vicinity.origin
    }
    fn block_hash(&self, _number: U256) -> H256 {
        self.vicinity.block_hash
    }
    fn block_number(&self) -> U256 {
        self.vicinity.block_number
    }
    fn block_coinbase(&self) -> H160 {
        self.vicinity.block_coinbase
    }
    fn block_timestamp(&self) -> U256 {
        self.vicinity.block_timestamp
    }
    fn block_difficulty(&self) -> U256 {
        self.vicinity.block_difficulty
    }
    fn block_gas_limit(&self) -> U256 {
        self.vicinity.block_gas_limit
    }
    fn block_base_fee_per_gas(&self) -> U256 {
        self.vicinity.block_base_fee_per_gas
    }

    fn chain_id(&self) -> U256 {
        self.vicinity.chain_id
    }

    fn exists(&self, address: H160) -> bool {
        let res = self.accounts.get(&address);

        match res {
            Ok(e) => e.is_some(),
            Err(e) => {
                log::error!("Got error from accounts: {:?}", e);
                false
            }
        }
    }

    fn basic(&self, address: H160) -> Basic {
        match self._basic(address) {
            Ok(e) => e,
            Err(e) => {
                log::error!("Faild to get basic: {:?}", e);
                Basic::default()
            }
        }
    }

    fn code(&self, address: H160) -> Vec<u8> {
        match self.accounts.get(&address) {
            Ok(Some(e)) => e.code.clone(),
            Ok(None) => Vec::new(),
            Err(e) => {
                log::error!("Faild to get basic: {:?}", e);
                Vec::new()
            }
        }
    }

    fn storage(&self, address: H160, index: H256) -> H256 {
        match self.storages.get(&address, &index) {
            Ok(Some(e)) => *e,
            Ok(None) => H256::default(),
            Err(e) => {
                log::error!("Faild to get basic: {:?}", e);
                H256::default()
            }
        }
    }

    fn original_storage(&self, address: H160, index: H256) -> Option<H256> {
        Some(self.storage(address, index))
    }
}
