use std::fs;
use bs3::backend::SledBackend;
use fm_coinbase::CoinbaseModule;
use fm_utxo::UtxoModule;
use libfindora::transaction::Transaction;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use sha3::Sha3_512;
use std::marker::PhantomData;
use zei::setup::PublicParams;
use fm_staking::StakingModule;
use crate::entry::DevOperation;

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

pub fn start(operation: DevOperation) {


    let path_vec = match operation {
        DevOperation::Single(path) => {
            vec![path]
        }
        DevOperation::Pos(v) => {
            v
        }
    };

    let f = |abcf_path:String,node:String|{

        // read config.toml
        let config = fs::read_to_string(abcf_path.clone());
        if config.is_err() {
            panic!("{}",config.unwrap_err().to_string());
        }

        let config = config.unwrap();

        let orig_cfg = [
            "index_all_keys = false",
            "laddr = \"tcp://127.0.0.1:26657\"",
            "timeout_propose = \"3s\"",
            "timeout_propose_delta = \"500ms\"",
            "timeout_prevote = \"1s\"",
            "timeout_prevote_delta = \"500ms\"",
            "timeout_precommit = \"1s\"",
            "timeout_precommit_delta = \"500ms\"",
            "timeout_commit = \"1s\"",
            "recheck = true",
            "fast_sync = true",
            "size = 5000",
            "prometheus = false",
        ];

        let target_cfg = [
            "index_all_keys = true",
            &format!("laddr = \"tcp://0.0.0.0:{}\"",node.clone()),
            "timeout_propose = \"8s\"",
            "timeout_propose_delta = \"100ms\"",
            "timeout_prevote = \"4s\"",
            "timeout_prevote_delta = \"100ms\"",
            "timeout_precommit = \"4s\"",
            "timeout_precommit_delta = \"100ms\"",
            "timeout_commit = \"15s\"",
            "recheck = false",
            "fast_sync = false",
            "size = 2000",
            "prometheus = false",
        ];

        // replace config.toml field
        let config = orig_cfg
            .iter()
            .zip(target_cfg.iter())
            .fold(config, |acc, pair| acc.replace(pair.0, pair.1));

        let result = fs::write(abcf_path.clone(), config);

        if result.is_err() {
            panic!("{}",result.unwrap_err().to_string());
        }

    };

    for (path,node_str) in path_vec.iter() {
        let staking_path = path.clone() + "/staking";
        let coinbase_path = path.clone() + "/coinbase";
        let utxo_path = path.clone() + "/utxo";
        let abcf_path = path.clone() + "/abcf";

        let staking = StakingModule::new(Vec::new());

        let coinbase = CoinbaseModule::new();

        let params = PublicParams::default();

        let prng = ChaChaRng::from_entropy();

        let utxo = UtxoModule::new(params, prng);

        let manager = Findorad::<SledBackend>::new(staking, coinbase, utxo);

        let staking_backend = bs3::backend::sled_db_open(Some(&*staking_path)).unwrap();
        let coinbase_backend = bs3::backend::sled_db_open(Some(&*coinbase_path)).unwrap();
        let utxo_backend = bs3::backend::sled_db_open(Some(&*utxo_path)).unwrap();

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
        let node = abcf_node::Node::new(entry, &*abcf_path).unwrap();

        f(abcf_path+"/config/config.toml",node_str.clone());
        node.start().unwrap();
    }


    std::thread::park();
}