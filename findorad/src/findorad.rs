use abcf::{tm_protos::abci::RequestDeliverTx, ToBytes};
use bs3::backend::SledBackend;
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
    #[dependence(coinbase = "coinbase")]
    pub staking: StakingModule,
    pub asset: AssetModule,
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
    pub fn new(prefix:Option<&str>) -> Self {

        let prefix_path = if let Some(prefix) = prefix {
            prefix.to_string()
        } else {
            "./target/findorad".to_string()
        };
        println!("prefix_path:{}",prefix_path);
        let staking = StakingModule::new(BTreeMap::new());

        let asset = AssetModule::new();

        let fee = FeeModule::new();

        let coinbase = CoinbaseModule::new(0);

        let params = PublicParams::default();
        let prng = ChaChaRng::from_entropy();
        let utxo = UtxoModule::new(params, prng);

        let manager = FindoradManager::<SledBackend>::new(staking, asset, fee, coinbase, utxo);

        let staking_backend =
            bs3::backend::sled_db_open(Some(format!("{}/{}",prefix_path,"staking").as_str())).unwrap();
        let coinbase_backend =
            bs3::backend::sled_db_open(Some(format!("{}/{}",prefix_path,"coinbase").as_str())).unwrap();
        let utxo_backend = bs3::backend::sled_db_open(Some(format!("{}/{}",prefix_path,"utxo").as_str())).unwrap();
        let asset_backend = bs3::backend::sled_db_open(Some(format!("{}/{}",prefix_path,"asset").as_str())).unwrap();
        let fee_backend = bs3::backend::sled_db_open(Some(format!("{}/{}",prefix_path,"fee").as_str())).unwrap();

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
        };

        let entry = abcf::entry::Node::new(stateless, stateful, manager);
        let node = abcf_node::Node::new(
            entry,
            format!("{}/{}",prefix_path,"abcf").as_str(),
            abcf_node::NodeType::Validator,
        )
        .unwrap();

        Self { node }
    }

    pub fn genesis(&mut self, tx: Transaction) -> Result<()> {
        use tm_abci::Application;

        let bytes = tx.to_bytes()?;
        let req = RequestDeliverTx { tx: bytes };

        let rt = tokio::runtime::Runtime::new().unwrap();

        let resp = rt.block_on(async { self.node.app.deliver_tx(req).await });
        log::info!("{:?}", resp);
        Ok(())
    }

    pub fn start(self) {
        self.node.start().unwrap();
        std::thread::park();
    }
}
