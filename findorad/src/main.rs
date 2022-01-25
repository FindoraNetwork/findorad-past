#![feature(generic_associated_types)]

// pub mod coinbase;
// pub mod utxo;

use bs3::backend::SledBackend;
use libfindora::transaction::Transaction;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use sha3::Sha3_512;
use std::{collections::BTreeMap, marker::PhantomData};
use tendermint_sys::NodeType;
use zei::setup::PublicParams;

use fm_asset::AssetModule;
use fm_coinbase::CoinbaseModule;
use fm_fee::FeeModule;
use fm_staking::StakingModule;
use fm_utxo::UtxoModule;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "Transaction"
)]
pub struct Findorad {
    #[dependence(coinbase = "coinbase")]
    pub staking: StakingModule,
    pub asset: AssetModule,
    pub fee: FeeModule,
    #[dependence(utxo = "utxo")]
    pub coinbase: CoinbaseModule,
    pub utxo: UtxoModule,
}

fn main() {
    env_logger::init();

    let staking = StakingModule::new(BTreeMap::new());

    let asset = AssetModule::new();

    let fee = FeeModule::new();

    let coinbase = CoinbaseModule::new(0);

    let params = PublicParams::default();
    let prng = ChaChaRng::from_entropy();
    let utxo = UtxoModule::new(params, prng);

    let manager = Findorad::<SledBackend>::new(staking, asset, fee, coinbase, utxo);

    let asset_backend = bs3::backend::sled_db_open(Some("./target/findorad/asset")).unwrap();
    let fee_backend = bs3::backend::sled_db_open(Some("./target/findorad/fee")).unwrap();
    let staking_backend = bs3::backend::sled_db_open(Some("./target/findorad/staking")).unwrap();
    let coinbase_backend = bs3::backend::sled_db_open(Some("./target/findorad/coinbase")).unwrap();
    let utxo_backend = bs3::backend::sled_db_open(Some("./target/findorad/utxo")).unwrap();

    let stateful = abcf::Stateful::<Findorad<SledBackend>> {
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
                SledBackend::open_tree(&staking_backend, "validator_addr_pubkey").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        asset: abcf::Stateful::<AssetModule<SledBackend, Sha3_512>> {
            asset_infos: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&asset_backend, "validator_addr_pubkey").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        fee: abcf::Stateful::<FeeModule<SledBackend, Sha3_512>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&fee_backend, "pending_outputs").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        coinbase: abcf::Stateful::<CoinbaseModule<SledBackend, Sha3_512>> {
            pending_outputs: abcf::bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&coinbase_backend, "pending_outputs").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        utxo: abcf::Stateful::<UtxoModule<SledBackend, Sha3_512>> {
            outputs_set: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&coinbase_backend, "output_set").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
    };

    let stateless = abcf::Stateless::<Findorad<SledBackend>> {
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
                SledBackend::open_tree(&asset_backend, "asset_infos").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        fee: abcf::Stateless::<FeeModule<SledBackend, Sha3_512>> {
            sl_value: abcf::bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&fee_backend, "pending_outputs").unwrap(),
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
    let node = abcf_node::Node::new(entry, "./target/findorad/abcf", NodeType::Validator).unwrap();
    node.start().unwrap();
    std::thread::park();
}
