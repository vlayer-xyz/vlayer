use call_server::{gas_meter::Config as GasMeterConfig, ConfigBuilder, ProofMode};
use common::GuestElf;
use derive_new::new;
use mock::{Anvil, Client, Contract, Server};
use mock_chain_server::ChainProofServerMock;
use server_utils::rpc::mock::Server as RpcServerMock;

pub(crate) fn call_guest_elf() -> GuestElf {
    call_guest_wrapper::GUEST_ELF.clone()
}

pub(crate) fn chain_guest_elf() -> GuestElf {
    chain_guest_wrapper::GUEST_ELF.clone()
}

pub(crate) const API_VERSION: &str = "1.2.3";
pub(crate) const GAS_METER_TTL: u64 = 3600;

#[derive(new)]
pub(crate) struct Context {
    client: Client,
    anvil: Anvil,
    chain_proof_server: ChainProofServerMock,
    gas_meter_server: Option<RpcServerMock>,
}

impl Context {
    pub(crate) async fn default() -> Self {
        let anvil = Anvil::start();
        let client = anvil.setup_client();
        let chain_proof_server = ChainProofServerMock::start().await;
        let mut ctx = Self::new(client, anvil, chain_proof_server, None);
        ctx.mock_latest_block().await;
        ctx
    }

    pub(crate) fn with_gas_meter_server(mut self, gas_meter_server: RpcServerMock) -> Self {
        self.gas_meter_server = Some(gas_meter_server);
        self
    }

    pub(crate) fn assert_gas_meter(&self) {
        self.gas_meter_server
            .as_ref()
            .expect("gas meter server not set up")
            .assert();
    }

    async fn mock_latest_block(&mut self) {
        let block_header = self.client.get_latest_block_header().await;
        self.chain_proof_server
            .mock_single_block(self.anvil.chain_id(), block_header)
            .await;
    }

    pub(crate) async fn deploy_contract(&mut self) -> Contract {
        let contract = self.client.deploy_contract().await;
        // Latest block must be updated in chain proof server, because otherwise host
        // would get a start execution location on block 0 without contract deployed
        self.mock_latest_block().await;
        contract
    }

    pub(crate) fn server(&self, call_guest_elf: GuestElf, chain_guest_elf: GuestElf) -> Server {
        let gas_meter_config = self
            .gas_meter_server
            .as_ref()
            .map(RpcServerMock::url)
            .map(|url| GasMeterConfig::new(url, GAS_METER_TTL, None));
        let config = ConfigBuilder::new(call_guest_elf, chain_guest_elf, API_VERSION.into())
            .with_chain_proof_url(self.chain_proof_server.url())
            .with_rpc_mappings([(self.anvil.chain_id(), self.anvil.endpoint())])
            .with_proof_mode(ProofMode::Fake)
            .with_gas_meter_config(gas_meter_config)
            .build();
        Server::new(config)
    }
}

pub(crate) mod mock {
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

    pub(crate) type Contract =
        ExampleProver<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>;

    pub(crate) struct Server(Router);

    impl Server {
        pub(crate) fn new(config: Config) -> Self {
            Self(server(config))
        }

        pub(crate) async fn post(&self, url: &str, body: impl Serialize) -> Response<Body> {
            post(self.0.clone(), url, &body).await
        }
    }

    #[derive(Deref)]
    pub(crate) struct Anvil(AnvilInstance);

    impl Anvil {
        pub(crate) fn start() -> Self {
            Self(utils::Anvil::new().chain_id(11_155_111_u64).spawn())
        }

        pub(crate) fn setup_client(&self) -> Client {
            let wallet: LocalWallet = self.keys()[0].clone().into();
            let provider = Provider::<Http>::try_from(self.endpoint())
                .unwrap()
                .interval(Duration::from_millis(10_u64));
            Client::new(provider, wallet.with_chain_id(self.chain_id()))
        }
    }

    #[derive(Deref)]
    pub(crate) struct Client(Arc<SignerMiddleware<Provider<Http>, Wallet<ecdsa::SigningKey>>>);

    impl Client {
        pub(crate) fn new(provider: Provider<Http>, wallet: Wallet<ecdsa::SigningKey>) -> Self {
            Client(Arc::new(SignerMiddleware::new(provider, wallet)))
        }

        pub(crate) async fn deploy_contract(&self) -> Contract {
            ExampleProver::deploy(self.0.clone(), ())
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
}
