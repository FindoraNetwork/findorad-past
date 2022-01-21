use abcf::tm_protos::abci::RequestDeliverTx;
use abcf::ToBytes;
use libfindora::asset::FRA;
use libfn::entity::Entity::{Define as EDefine, Issue as EIssue};
use libfn::entity::{Define, Issue};
use libfn::types::Wallet;
use libfn::Builder;
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

pub fn define_issue_fra(entry: &mut dyn tm_abci::Application) {
    // Address:            0x715732da92526f2506234916a209e506700402dc (ETH Compatible)
    // Findora Address:    fra18ljye04gkwlc3f3lv86vwvjdwqrswwhan6qld3sdvmr29gpsmw3sg590ru
    // Findora Public Key: P+RMvqizv4imP2H0xzJNcAcHOv2egfbGDWbGoqAw26M=
    // Secret:             cPQpsjGCb6GexLmywO5QHz0x7QTxZIFXUG6kLhlruGY=
    // Mnemonic:
    // sport cupboard crumble perfect bubble sad flight divert silk hope high wood tip various mystery pizza foster special solid tail client deputy fine tackle

    let wallet = Wallet::from_mnemonic("sport cupboard crumble perfect bubble sad flight divert silk hope high wood tip various mystery pizza foster special solid tail client deputy fine tackle").unwrap();
    let kp = wallet.secret.key.into_keypair();

    let define_entry = EDefine(Define {
        maximum: Some(U256::from(1000 + 210_0000_0000 * FRA.units)),
        transferable: true,
        keypair: kp.clone(),
        asset: FRA.bare_asset_type,
    });

    let issue_entry = EIssue(Issue {
        amount: 210_0000_0000 * FRA.units / 2,
        asset_type: FRA.bare_asset_type,
        confidential_amount: false,
        keypair: kp,
    });

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let mut p = EmptyP {};
        let mut prng = ChaChaRng::from_entropy();
        let mut builder = Builder::default();
        builder
            .from_entities(&mut prng, &mut p, vec![define_entry, issue_entry])
            .await
            .unwrap();
        let tx = builder.build(&mut prng).unwrap();
        let tx = tx.to_bytes().unwrap();
        let req_tx = RequestDeliverTx { tx };

        let r = entry.deliver_tx(req_tx).await;

        log::debug!("code:{:?}", r.code);
        log::debug!("log:{:?}", r.log);
        log::debug!("info:{:?}", r.info);
        log::debug!("codespace:{:?}", r.codespace);
    });
}
