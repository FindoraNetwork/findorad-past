#![feature(generic_associated_types)]

// pub mod coinbase;
// pub mod utxo;

use std::marker::PhantomData;
use bs3::backend::SledBackend;
use libfindora::transaction::Transaction;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use sha3::Sha3_512;
use zei::setup::PublicParams;
use fm_utxo::UtxoModule;
use fm_coinbase::CoinbaseModule;

#[abcf::manager(
    name = "findorad",
    digest = "sha3::Sha3_512",
    version = 0,
    impl_version = "1.0.0",
    transaction = "Transaction"
)]
pub struct Findorad {
    pub coinbase: CoinbaseModule,
    pub utxo: UtxoModule,
}

fn main() {
    env_logger::init();

    let coinbase = CoinbaseModule::new();

    let params = PublicParams::default();

    let prng = ChaChaRng::from_entropy();

    let utxo = UtxoModule::new(params, prng);

    let manager = Findorad::<SledBackend>::new(coinbase, utxo);

    let coinbase_backend = bs3::backend::sled_db_open(Some("./target/findorad/coinbase")).unwrap();
    let utxo_backend = bs3::backend::sled_db_open(Some("./target/findorad/utxo")).unwrap();

    let stateful = abcf::Stateful::<Findorad<SledBackend>> {
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
