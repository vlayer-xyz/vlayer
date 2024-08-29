use axum::{body::Body, http::Response};
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
use example_prover::ExampleProver;
use serde::Serialize;
use serde_json::json;
use server_utils::post;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

pub(crate) struct TestHelper {
    anvil: AnvilInstance,
    pub(crate) block_number: u32,
    pub(crate) contract: ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>,
}

impl TestHelper {
    pub(crate) async fn create() -> Self {
        let anvil = setup_anvil().await;
        let client = setup_client(&anvil).await;
        let contract = deploy_test_contract(client).await;
        let block_number = set_block_nr(&anvil).await;

        Self {
            anvil,
            block_number,
            contract,
        }
    }

    pub(crate) async fn post<T: Serialize>(&self, url: &str, body: &T) -> Response<Body> {
        let app = server(ServerConfig {
            rpc_urls: HashMap::from([(self.anvil.chain_id(), self.anvil.endpoint())]),
            port: 3000,
            proof_mode: ProofMode::Fake,
        });
        post(app, url, body).await
    }
}

async fn setup_anvil() -> AnvilInstance {
    Anvil::new().chain_id(11155111u64).spawn()
}

async fn setup_client(
    anvil: &AnvilInstance,
) -> Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>> {
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));
    Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(anvil.chain_id()),
    ))
}

async fn deploy_test_contract(
    client: Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>,
) -> ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>> {
    ExampleProver::deploy(client, ())
        .unwrap()
        .send()
        .await
        .unwrap()
}

async fn set_block_nr(anvil: &AnvilInstance) -> u32 {
    let req = json!({
        "jsonrpc": "2.0",
        "method": "eth_blockNumber",
        "params": [],
        "id": 0
    });

    let response = reqwest::Client::new()
        .post(anvil.endpoint())
        .json(&req)
        .send()
        .await
        .unwrap();

    let body = response.text().await.unwrap();
    let json: serde_json::Value = serde_json::from_str(&body).unwrap();
    let result = json["result"].clone();
    let result = result.as_str().unwrap();
    u32::from_str_radix(&result[2..], 16).unwrap()
}
