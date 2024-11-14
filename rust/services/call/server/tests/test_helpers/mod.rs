use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::{body::Body, http::Response};
use block_header::EvmBlockHeader;
use call_guest_wrapper::GUEST_ELF;
use call_server::{server, ProofMode, ServerConfig};
use ethers::{
    contract::abigen,
    core::{
        k256::ecdsa,
        utils::{Anvil, AnvilInstance},
    },
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer, Wallet},
    types::BlockNumber as BlockTag,
};
use example_prover::ExampleProver;
use mock_chain_server::{fake_proof_result, ChainProofServerMock};
use provider::to_eth_block_header;
use serde::Serialize;
use serde_json::json;
use server_utils::post;

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

type Client = Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;
type Contract = ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;

pub(crate) struct TestHelper {
    anvil: AnvilInstance,
    client: Client,
    pub(crate) contract: Contract,
}

impl TestHelper {
    pub(crate) async fn create() -> Self {
        let anvil = setup_anvil().await;
        let client = setup_client(&anvil).await;
        let contract = deploy_test_contract(client.clone()).await;

        Self {
            anvil,
            client,
            contract,
        }
    }

    pub(crate) async fn post<T: Serialize>(&self, url: &str, body: &T) -> Response<Body> {
        let latest_block_header = get_latest_block_header(&self.client).await;
        let chain_proof_server_mock = start_chain_proof_server(latest_block_header).await;

        let app = server(ServerConfig {
            rpc_urls: HashMap::from([(self.anvil.chain_id(), self.anvil.endpoint())]),
            proof_mode: ProofMode::Fake,
            chain_proof_url: chain_proof_server_mock.url(),
            call_guest: GUEST_ELF.clone(),
            ..ServerConfig::default()
        });
        post(app, url, body).await
    }
}

async fn start_chain_proof_server(
    latest_block_header: Box<dyn EvmBlockHeader>,
) -> ChainProofServerMock {
    let result = fake_proof_result(latest_block_header);
    ChainProofServerMock::start(json!({}), result).await
}

async fn setup_anvil() -> AnvilInstance {
    Anvil::new().chain_id(11155111u64).spawn()
}

async fn setup_client(anvil: &AnvilInstance) -> Client {
    let wallet: LocalWallet = anvil.keys()[0].clone().into();
    let provider = Provider::<Http>::try_from(anvil.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));
    Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id())))
}

async fn deploy_test_contract(client: Client) -> Contract {
    ExampleProver::deploy(client, ())
        .unwrap()
        .send()
        .await
        .unwrap()
}

async fn get_latest_block_header(client: &Client) -> Box<dyn EvmBlockHeader> {
    let latest_block = client
        .as_ref()
        .get_block(BlockTag::Latest)
        .await
        .unwrap()
        .unwrap();
    let block_header = to_eth_block_header(latest_block).unwrap();
    Box::new(block_header)
}
