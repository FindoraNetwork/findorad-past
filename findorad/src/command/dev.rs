use libfindora::{asset::FRA, Transaction};
use libfn::{
    entity::{Define, Entity, Issue},
    types::Wallet,
};
use primitive_types::U256;
use rand_chacha::ChaChaRng;
use rand_core::SeedableRng;
use serde::{Deserialize, Serialize};

pub struct EmptyP {}

#[async_trait::async_trait]
impl abcf_sdk::providers::Provider for EmptyP {
    async fn request<Req, Resp>(
        &mut self,
        _method: &str,
        _params: &Req,
    ) -> abcf_sdk::error::Result<Option<Resp>>
    where
        Req: Serialize + Send + Sync,
        Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        Ok(None)
    }

    async fn receive(&mut self) -> abcf_sdk::error::Result<Option<String>> {
        Ok(None)
    }
}

pub fn define_issue_fra() -> Transaction {
    // ETH Compatible Address: 0x283590e19dee343ea0a8f4ecec906d53308068b5
    // Findora Address:        fra11wh7g4syk4xfufr8fk58zsdvsuxw7udp7pr5gn8
    // Findora Public Key:     7C2budB1QtXfoRJ-g-GN4BMdXpgkLv7MtTXG2yC3K3Q=
    // Secret:                 _12euPXJxDbpcw7fMNJufUZgrTgcK7ShTJmXuZZe8eM=
    // Mnemonic:
    // dentist earth learn way nominee satisfy scorpion curious gate chapter draw river broom tenant empower ordinary grunt window horn balance stone marble flat found

    let wallet = Wallet::from_mnemonic("dentist earth learn way nominee satisfy scorpion curious gate chapter draw river broom tenant empower ordinary grunt window horn balance stone marble flat found").unwrap();
    let kp = wallet.secret.key.into_keypair();

    let define_entry = Entity::Define(Define {
        maximum: Some(U256::from(1000 + 210_0000_0000 * FRA.units)),
        transferable: true,
        keypair: kp.clone(),
        asset: FRA.bare_asset_type,
    });

    let issue_entry = Entity::Issue(Issue {
        amount: 210_0000_0000 * FRA.units,
        asset_type: FRA.bare_asset_type,
        confidential_amount: false,
        keypair: kp,
    });

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut p = EmptyP {};
        let mut prng = ChaChaRng::from_entropy();
        let mut builder = libfn::Builder::default();
        builder
            .from_entities(&mut prng, &mut p, vec![define_entry, issue_entry])
            .await
            .unwrap();
        builder.build(&mut prng).unwrap()
    })
}
