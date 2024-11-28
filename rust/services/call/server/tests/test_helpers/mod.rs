use std::{sync::Arc, time::Duration};

use axum::{body::Body, http::Response, Router};
use block_header::EvmBlockHeader;
use call_server::{gas_meter::Config as GasMeterConfig, server, Config, ConfigBuilder, ProofMode};
use common::GuestElf;
use derive_more::Deref;
use derive_new::new;
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
use server_utils::{post, RpcServerMock};

abigen!(ExampleProver, "./testdata/ExampleProver.json",);

type Client = Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;
type Contract = ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;

pub(crate) fn default_call_guest_elf() -> GuestElf {
    call_guest_wrapper::GUEST_ELF.clone()
}

pub(crate) fn default_chain_guest_elf() -> GuestElf {
    chain_guest_wrapper::GUEST_ELF.clone()
}

pub(crate) const DEFAULT_API_VERSION: &str = "1.2.3";
pub(crate) const DEFAULT_GAS_METER_TTL: u64 = 3600;

#[derive(new)]
pub(crate) struct Context {
    pub(crate) client: ClientMock,
    pub(crate) anvil: AnvilMock,
    pub(crate) chain_proof_server: ChainProofServerMock,
    pub(crate) gas_meter_server: Option<RpcServerMock>,
}

impl Context {
    pub(crate) async fn default() -> Self {
        let anvil = AnvilMock::start().await;
        let client = anvil.setup_client().await;
        let block_header = client.get_latest_block_header().await;
        let chain_proof_server =
            ChainProofServerMock::start(json!({}), fake_proof_result(block_header), 1).await;
        Self::new(client, anvil, chain_proof_server, None)
    }

    pub(crate) fn server(&self, call_guest_elf: GuestElf, chain_guest_elf: GuestElf) -> ServerMock {
        let mut config_builder = ConfigBuilder::new(
            self.chain_proof_server.url(),
            call_guest_elf,
            chain_guest_elf,
            DEFAULT_API_VERSION.into(),
        )
        .with_rpc_mappings([(self.anvil.chain_id(), self.anvil.endpoint())])
        .with_proof_mode(ProofMode::Fake);

        if let Some(url) = self.gas_meter_server.as_ref().map(|x| x.url()) {
            config_builder = config_builder
                .with_gas_meter_config(GasMeterConfig::new(url, DEFAULT_GAS_METER_TTL));
        }

        let config = config_builder.build();
        ServerMock::new(config)
    }
}

pub(crate) struct ServerMock(Router);

impl ServerMock {
    pub(crate) fn new(config: Config) -> Self {
        Self(server(config))
    }

    pub(crate) async fn post(&self, url: &str, body: impl Serialize) -> Response<Body> {
        post(self.0.clone(), url, &body).await
    }
}

#[derive(Deref)]
pub(crate) struct AnvilMock(AnvilInstance);

impl AnvilMock {
    pub(crate) async fn start() -> Self {
        AnvilMock(Anvil::new().chain_id(11155111u64).spawn())
    }

    pub(crate) async fn setup_client(&self) -> ClientMock {
        let wallet: LocalWallet = self.keys()[0].clone().into();
        let provider = Provider::<Http>::try_from(self.endpoint())
            .unwrap()
            .interval(Duration::from_millis(10u64));
        ClientMock(Arc::new(SignerMiddleware::new(provider, wallet.with_chain_id(self.chain_id()))))
    }
}

#[derive(Deref)]
pub(crate) struct ClientMock(Client);

impl ClientMock {
    pub(crate) async fn deploy_contract(self) -> Contract {
        ExampleProver::deploy(self.0, ())
            .unwrap()
            .send()
            .await
            .unwrap()
    }

    pub(crate) async fn get_latest_block_header(&self) -> Box<dyn EvmBlockHeader> {
        let latest_block = self
            .as_ref()
            .get_block(BlockTag::Latest)
            .await
            .unwrap()
            .unwrap();
        let block_header = to_eth_block_header(latest_block).unwrap();
        Box::new(block_header)
    }
}
