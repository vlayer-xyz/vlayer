use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, Response},
};
use call_server::server;
use call_server::{ProofMode, ServerConfig};
use ethers::{
    contract::abigen,
    core::{
        k256::ecdsa,
        utils::{Anvil, AnvilInstance},
    },
    middleware::SignerMiddleware,
    providers::{Http, Provider},
    signers::{LocalWallet, Signer, Wallet},
};
use mime::APPLICATION_JSON;
use serde::Serialize;
use serde_json::json;
use serde_json::to_string;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

#[derive(Default)]
pub(crate) struct TestHelper {
    client: Option<Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>>,
    anvil: Option<AnvilInstance>,
    pub(crate) block_number: u32,
    pub(crate) contract: Option<
        example_prover::ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>,
    >,
}

pub(crate) async fn test_helper() -> TestHelper {
    let mut test_helper = TestHelper::default();
    test_helper.setup_anvil().await;
    test_helper.deploy_test_contract().await;
    test_helper.set_block_nr().await;
    test_helper
}

impl TestHelper {
    pub(crate) fn anvil(&self) -> &AnvilInstance {
        self.anvil.as_ref().unwrap()
    }

    pub(crate) fn contract(
        &self,
    ) -> example_prover::ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>
    {
        self.contract.as_ref().unwrap().clone()
    }

    fn client(&self) -> Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>> {
        self.client.as_ref().unwrap().clone()
    }

    pub(crate) async fn post<T>(&self, url: &str, body: &T) -> anyhow::Result<Response<Body>>
    where
        T: Serialize,
    {
        let app = server(ServerConfig {
            rpc_urls: HashMap::from([(self.anvil().chain_id(), self.anvil().endpoint())]),
            port: 3000,
            proof_mode: ProofMode::Fake,
        });
        let request = Request::post(url)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .body(Body::from(to_string(body)?))?;
        Ok(app.oneshot(request).await?)
    }

    async fn setup_anvil(&mut self) {
        self.anvil = Some(Anvil::new().chain_id(11155111u64).spawn());
        let wallet: LocalWallet = self.anvil().keys()[0].clone().into();
        let provider = Provider::<Http>::try_from(self.anvil().endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));
        let client = Arc::new(SignerMiddleware::new(
            provider,
            wallet.with_chain_id(self.anvil().chain_id()),
        ));
        self.client = Some(client.clone());
    }

    async fn deploy_test_contract(&mut self) {
        let example_contract = ExampleProver::deploy(self.client(), ())
            .unwrap()
            .send()
            .await
            .unwrap();
        self.contract = Some(example_contract);
    }

    async fn set_block_nr(&mut self) {
        let req = json!({
            "jsonrpc": "2.0",
            "method": "eth_blockNumber",
            "params": [],
            "id": 0
        });

        let response = reqwest::Client::new()
            .post(self.anvil().endpoint())
            .json(&req)
            .send()
            .await
            .unwrap();

        let body = response.text().await.unwrap();
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        let result = json["result"].clone();
        let result = result.as_str().unwrap();
        self.block_number = u32::from_str_radix(&result[2..], 16).unwrap();
    }
}
