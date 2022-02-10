use abcf::{tm_protos::abci::RequestDeliverTx, ToBytes};
use bs3::backend::SledBackend;
use fm_chain::ChainModule;
use fm_evm::EvmModule;
use libfindora::transaction::Transaction;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use sha3::Sha3_512;
use std::{collections::BTreeMap, marker::PhantomData};
use zei::setup::PublicParams;

use fm_asset::AssetModule;
use fm_coinbase::CoinbaseModule;
use fm_fee::FeeModule;
use fm_staking::StakingModule;
use fm_utxo::UtxoModule;

use crate::Result;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "Transaction"
)]
pub struct FindoradManager {
    pub chain: ChainModule,
    #[dependence(coinbase = "coinbase")]
    pub staking: StakingModule,
    pub asset: AssetModule,
    #[dependence(utxo = "utxo", chain = "chain")]
    pub evm: EvmModule,
    pub fee: FeeModule,
    #[dependence(utxo = "utxo")]
    pub coinbase: CoinbaseModule,
    pub utxo: UtxoModule,
}

type FindoradManagerWithSled = FindoradManager<SledBackend>;

pub struct Findorad {
    node: abcf_node::Node<abcf::entry::Node<sha3::Sha3_512, FindoradManagerWithSled>>,
}

impl Findorad {
    pub fn new() -> Self {
        let staking = StakingModule::new(BTreeMap::new());

        let asset = AssetModule::new();

        let evm = EvmModule::new(fm_evm::evm::vicinity::Vicinity::mainnet());

        let fee = FeeModule::new(0);

        let coinbase = CoinbaseModule::new(0);

        let params = PublicParams::default();

        let prng = ChaChaRng::from_entropy();

        let utxo = UtxoModule::new(params, prng);

        let manager = FindoradManager::<SledBackend>::new(staking, asset, evm, fee, coinbase, utxo);

        let staking_backend =
            bs3::backend::sled_db_open(Some("./target/findorad/staking")).unwrap();
        let coinbase_backend =
            bs3::backend::sled_db_open(Some("./target/findorad/coinbase")).unwrap();
        let utxo_backend = bs3::backend::sled_db_open(Some("./target/findorad/utxo")).unwrap();
        let asset_backend = bs3::backend::sled_db_open(Some("./target/findorad/asset")).unwrap();
        let fee_backend = bs3::backend::sled_db_open(Some("./target/findorad/fee")).unwrap();
        let evm_backend = bs3::backend::sled_db_open(Some("./target/findorad/evm")).unwrap();

        let stateful = abcf::Stateful::<FindoradManager<SledBackend>> {
            staking: abcf::Stateful::<StakingModule<SledBackend, Sha3_512>> {
                global_power: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "global_power").unwrap(),
                )
                .unwrap(),
                delegators: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "delegators").unwrap(),
                )
                .unwrap(),
                powers: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "powers").unwrap(),
                )
                .unwrap(),
                validator_staker: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "validator_staker").unwrap(),
                )
                .unwrap(),
                validator_pubkey: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "validator_pubkey").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            asset: abcf::Stateful::<AssetModule<SledBackend, Sha3_512>> {
                asset_infos: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&asset_backend, "asset_infos").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            evm: abcf::Stateful::<EvmModule<SledBackend, Sha3_512>> {
                accounts: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&evm_backend, "accounts").unwrap(),
                )
                .unwrap(),
                storages: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&evm_backend, "storages").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            fee: abcf::Stateful::<FeeModule<SledBackend, Sha3_512>> {
                sf_value: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&fee_backend, "sf_value").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            coinbase: abcf::Stateful::<CoinbaseModule<SledBackend, Sha3_512>> {
                pending_outputs: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&coinbase_backend, "coinbase").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            utxo: abcf::Stateful::<UtxoModule<SledBackend, Sha3_512>> {
                outputs_set: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&utxo_backend, "output_set").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
        };

        let stateless = abcf::Stateless::<FindoradManager<SledBackend>> {
            staking: abcf::Stateless::<StakingModule<SledBackend, Sha3_512>> {
                delegation_amount: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "delegation_amount").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            asset: abcf::Stateless::<AssetModule<SledBackend, Sha3_512>> {
                sl_value: bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&staking_backend, "sl_value").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            fee: abcf::Stateless::<FeeModule<SledBackend, Sha3_512>> {
                sl_value: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&fee_backend, "sl_value").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            coinbase: abcf::Stateless::<CoinbaseModule<SledBackend, Sha3_512>> {
                sl_value: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&coinbase_backend, "sl_value").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            utxo: abcf::Stateless::<UtxoModule<SledBackend, Sha3_512>> {
                owned_outputs: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&utxo_backend, "owned_outputs").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
            evm: abcf::Stateless::<EvmModule<SledBackend, Sha3_512>> {
                sl_value: abcf::bs3::SnapshotableStorage::new(
                    Default::default(),
                    SledBackend::open_tree(&evm_backend, "sl_value").unwrap(),
                )
                .unwrap(),
                __marker_s: PhantomData,
                __marker_d: PhantomData,
            },
        };

        let entry = abcf::entry::Node::new(stateless, stateful, manager);
        let node = abcf_node::Node::new(
            entry,
            "./target/findorad/abcf",
            abcf_node::NodeType::Validator,
        )
        .unwrap();

        Self { node }
    }

    pub fn genesis(&mut self, tx: Transaction) -> Result<()> {
        use tm_abci::Application;

        let height = self.node.app.height()?;

        if height == 0 {
            let bytes = tx.to_bytes()?;
            let req = RequestDeliverTx { tx: bytes };

            let rt = tokio::runtime::Runtime::new().unwrap();

            let resp = rt.block_on(async { self.node.app.deliver_tx(req).await });
            log::info!("{:?}", resp);
        }

        Ok(())
    }

    pub fn start(self) {
        self.node.start().unwrap();
        std::thread::park();
    }
}
