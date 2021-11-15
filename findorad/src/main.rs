#![feature(generic_associated_types)]

// pub mod coinbase;
// pub mod utxo;

use bs3::backend::SledBackend;
use fm_coinbase::CoinbaseModule;
use fm_staking::StakingModule;
use fm_utxo::UtxoModule;
use libfindora::transaction::Transaction;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use sha3::Sha3_512;
use std::marker::PhantomData;
use zei::setup::PublicParams;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "Transaction"
)]
pub struct Findorad {
    pub staking: StakingModule,
    pub coinbase: CoinbaseModule,
    pub utxo: UtxoModule,
}

fn main() {
    env_logger::init();

    let staking = StakingModule::new(Vec::new());

    let coinbase = CoinbaseModule::new();

    let params = PublicParams::default();

    let prng = ChaChaRng::from_entropy();

    let utxo = UtxoModule::new(params, prng);

    let manager = Findorad::<SledBackend>::new(staking, coinbase, utxo);

    let staking_backend = bs3::backend::sled_db_open(Some("./target/findorad/staking")).unwrap();
    let coinbase_backend = bs3::backend::sled_db_open(Some("./target/findorad/coinbase")).unwrap();
    let utxo_backend = bs3::backend::sled_db_open(Some("./target/findorad/utxo")).unwrap();

    let stateful = abcf::Stateful::<Findorad<SledBackend>> {
        staking: abcf::Stateful::<StakingModule<SledBackend>> {
            global_power: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&staking_backend, "global_power").unwrap(),
            )
            .unwrap(),
            delegation_amount: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&staking_backend, "delegation_amount").unwrap(),
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
            validator_addr_pubkey: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&staking_backend, "validator_addr_pubkey").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
        },
        coinbase: abcf::Stateful::<CoinbaseModule<SledBackend>> {
            asset_owner: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&coinbase_backend, "asset_owner").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
        },
        utxo: abcf::Stateful::<UtxoModule<SledBackend>> {
            output_set: bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&coinbase_backend, "output_set").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
        },
    };

    let stateless = abcf::Stateless::<Findorad<SledBackend>> {
        staking: abcf::Stateless::<StakingModule<SledBackend>> {
            sl_value: abcf::bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&staking_backend, "sl_value").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
        },
        coinbase: abcf::Stateless::<CoinbaseModule<SledBackend>> {
            sl_value: abcf::bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&utxo_backend, "sl_value").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
        },
        utxo: abcf::Stateless::<UtxoModule<SledBackend>> {
            owned_outputs: abcf::bs3::SnapshotableStorage::new(
                Default::default(),
                SledBackend::open_tree(&utxo_backend, "owned_outputs").unwrap(),
            )
            .unwrap(),
            __marker_s: PhantomData,
        },
    };

    let entry = abcf::entry::Node::new(stateless, stateful, manager);
    let node = abcf_node::Node::new(entry, "./target/findorad/abcf").unwrap();
    node.start().unwrap();
    std::thread::park();
}
