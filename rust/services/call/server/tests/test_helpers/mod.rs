use std::{collections::HashMap, sync::Arc, time::Duration};

use alloy_primitives::Bytes;
use axum::{body::Body, http::Response};
use call_server::{server, ProofMode, ServerConfig};
use chain_server::server::ChainProofServerMock;
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

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

pub(crate) struct TestHelper {
    anvil: AnvilInstance,
    pub(crate) contract: ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>,
}

impl TestHelper {
    pub(crate) async fn create() -> Self {
        let anvil = setup_anvil().await;
        let client = setup_client(&anvil).await;
        let contract = deploy_test_contract(client).await;

        Self { anvil, contract }
    }

    pub(crate) async fn post<T: Serialize>(&self, url: &str, body: &T) -> Response<Body> {
        let chain_proof_server_mock = ChainProofServerMock::start().await;
        chain_proof_server_mock
            .mock(
                json!({}),
                json!({
                    "proof": Bytes::default(),
                    "nodes": Vec::<Bytes>::default()
                }),
            )
            .await;

        let app = server(ServerConfig {
            rpc_urls: HashMap::from([(self.anvil.chain_id(), self.anvil.endpoint())]),
            host: "127.0.0.1".into(),
            port: 3000,
            proof_mode: ProofMode::Fake,
            chain_proof_url: chain_proof_server_mock.url(),
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
    Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id())))
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
