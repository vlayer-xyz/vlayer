use call_server::{gas_meter::Config as GasMeterConfig, ConfigBuilder, ProofMode};
use common::GuestElf;
use derive_new::new;
use mock::{Anvil, Client, Server};
use mock_chain_server::{fake_proof_result, ChainProofServerMock};
use serde_json::json;
use server_utils::rpc::mock::Server as RpcServerMock;

pub fn call_guest_elf() -> GuestElf {
    call_guest_wrapper::GUEST_ELF.clone()
}

pub fn chain_guest_elf() -> GuestElf {
    chain_guest_wrapper::GUEST_ELF.clone()
}

pub const API_VERSION: &str = "1.2.3";
pub const GAS_METER_TTL: u64 = 3600;

#[derive(new)]
pub struct Context {
    pub client: Client,
    pub anvil: Anvil,
    pub chain_proof_server: ChainProofServerMock,
    pub gas_meter_server: Option<RpcServerMock>,
}

impl Context {
    pub async fn default() -> Self {
        let anvil = Anvil::start();
        let client = anvil.setup_client();
        let block_header = client.get_latest_block_header().await;
        let chain_proof_server =
            ChainProofServerMock::start(json!({}), fake_proof_result(block_header), 1).await;
        Self::new(client, anvil, chain_proof_server, None)
    }

    pub fn server(&self, call_guest_elf: GuestElf, chain_guest_elf: GuestElf) -> Server {
        let mut config_builder = ConfigBuilder::new(
            self.chain_proof_server.url(),
            call_guest_elf,
            chain_guest_elf,
            API_VERSION.into(),
        )
        .with_rpc_mappings([(self.anvil.chain_id(), self.anvil.endpoint())])
        .with_proof_mode(ProofMode::Fake);

        if let Some(url) = self.gas_meter_server.as_ref().map(RpcServerMock::url) {
            config_builder =
                config_builder.with_gas_meter_config(GasMeterConfig::new(url, GAS_METER_TTL));
        }

        let config = config_builder.build();
        Server::new(config)
    }
}

pub mod mock {
    use std::{sync::Arc, time::Duration};

    use axum::{body::Body, http::Response, Router};
    use block_header::EvmBlockHeader;
    use call_server::{server, Config};
    use derive_more::Deref;
    use ethers::{
        contract::abigen,
        core::{
            k256::ecdsa,
            utils::{self, AnvilInstance},
        },
        middleware::{Middleware, SignerMiddleware},
        providers::{Http, Provider},
        signers::{LocalWallet, Signer, Wallet},
        types::BlockNumber as BlockTag,
    };
    use provider::to_eth_block_header;
    use serde::Serialize;
    use server_utils::post;

    abigen!(ExampleProver, "./testdata/ExampleProver.json");

    type Contract = ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;

    pub struct Server(Router);

    impl Server {
        pub fn new(config: Config) -> Self {
            Self(server(config))
        }

        pub async fn post(&self, url: &str, body: impl Serialize) -> Response<Body> {
            post(self.0.clone(), url, &body).await
        }
    }

    #[derive(Deref)]
    pub struct Anvil(AnvilInstance);

    impl Anvil {
        pub fn start() -> Self {
            Self(utils::Anvil::new().chain_id(11_155_111_u64).spawn())
        }

        pub fn setup_client(&self) -> Client {
            let wallet: LocalWallet = self.keys()[0].clone().into();
            let provider = Provider::<Http>::try_from(self.endpoint())
                .unwrap()
                .interval(Duration::from_millis(10_u64));
            Client::new(provider, wallet.with_chain_id(self.chain_id()))
        }
    }

    #[derive(Deref)]
    pub struct Client(Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>);

    impl Client {
        pub fn new(provider: Provider<Http>, wallet: Wallet<ecdsa::SigningKey>) -> Self {
            Client(Arc::new(SignerMiddleware::new(provider, wallet)))
        }

        pub async fn deploy_contract(self) -> Contract {
            ExampleProver::deploy(self.0, ())
                .unwrap()
                .send()
                .await
                .unwrap()
        }

        pub async fn get_latest_block_header(&self) -> Box<dyn EvmBlockHeader> {
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
}
