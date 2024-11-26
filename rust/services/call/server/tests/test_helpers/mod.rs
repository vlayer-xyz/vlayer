use std::{sync::Arc, time::Duration};

use axum::{body::Body, http::Response};
use block_header::EvmBlockHeader;
use call_guest_wrapper::GUEST_ELF as CALL_GUEST_ELF;
use call_server::{server, Config, GasMeterConfig, GasMeterServerMock, ProofMode};
use chain_guest_wrapper::GUEST_ELF as CHAIN_GUEST_ELF;
use common::GuestElf;
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

const DEFAULT_GAS_METER_TTL: u64 = 3600;

pub(crate) struct TestHelper {
    server_config: Config,
    pub(crate) contract: Contract,

    #[allow(dead_code)] // Keeps anvil alive
    anvil: AnvilInstance,
    #[allow(dead_code)] // Keeps chain proof server alive
    chain_proof_server_mock: ChainProofServerMock,
    #[allow(dead_code)]
    gas_meter_server_mock: GasMeterServerMock,
}

impl TestHelper {
    pub(crate) async fn default() -> Self {
        Self::new(CALL_GUEST_ELF.clone(), CHAIN_GUEST_ELF.clone()).await
    }

    pub(crate) async fn new(call_guest_elf: GuestElf, chain_guest_elf: GuestElf) -> Self {
        let anvil = setup_anvil().await;
        let client = setup_client(&anvil).await;
        let contract = deploy_test_contract(client.clone()).await;
        let latest_block_header = get_latest_block_header(&client).await;
        let chain_proof_server_mock = start_chain_proof_server(latest_block_header).await;
        let gas_meter_server_mock = start_gas_meter_server().await;

        let server_config = call_server::ConfigBuilder::new(
            chain_proof_server_mock.url(),
            call_guest_elf,
            chain_guest_elf,
        )
        .with_rpc_mappings([(anvil.chain_id(), anvil.endpoint())])
        .with_proof_mode(ProofMode::Fake)
        .with_gas_meter_config(GasMeterConfig {
            url: gas_meter_server_mock.url(),
            time_to_live: DEFAULT_GAS_METER_TTL,
        })
        .build();

        Self {
            server_config,
            contract,

            anvil,
            chain_proof_server_mock,
            gas_meter_server_mock,
        }
    }

    pub(crate) async fn post<T: Serialize>(&self, url: &str, body: &T) -> Response<Body> {
        let app = server(self.server_config.clone());
        post(app, url, body).await
    }
}

async fn start_gas_meter_server() -> GasMeterServerMock {
    GasMeterServerMock::start(json!({}), json!({})).await
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
