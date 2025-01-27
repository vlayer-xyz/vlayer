use std::time::Duration;

use call_server_lib::{ConfigBuilder, ProofMode};
use common::GuestElf;
use derive_new::new;
use ethers::types::{Bytes, H160};
use mock::{Anvil, ChainProofServer, Client, Contract, GasMeterServer, Server};
use serde_json::{json, Value};

pub const GAS_LIMIT: u64 = 1_000_000;
pub const ETHEREUM_SEPOLIA_ID: u64 = 11_155_111;
pub const GAS_METER_TTL: Duration = Duration::from_secs(3600);
pub const CHAIN_PROOF_POLL_INTERVAL: Duration = Duration::from_secs(5);
pub const CHAIN_PROOF_TIMEOUT: Duration = Duration::from_secs(120);

pub fn allocate_gas_body(expected_hash: &str) -> Value {
    json!({
        "gas_limit": GAS_LIMIT,
        "hash": expected_hash,
        "time_to_live": GAS_METER_TTL.as_secs(),
    })
}

pub fn v_call_body(contract_address: H160, call_data: &Bytes) -> Value {
    let params = json!([
        {
            "to": contract_address,
            "data": call_data,
            "gas_limit": GAS_LIMIT,
        },
        {
            "chain_id": ETHEREUM_SEPOLIA_ID,
        }
    ]);

    rpc_body("v_call", &params)
}

pub fn rpc_body(method: &str, params: &Value) -> Value {
    json!({
        "method": method,
        "params": params,
        "id": 1,
        "jsonrpc": "2.0",
    })
}

pub(crate) fn call_guest_elf() -> GuestElf {
    guest_wrapper::CALL_GUEST_ELF.clone()
}

pub(crate) fn chain_guest_elf() -> &'static GuestElf {
    &guest_wrapper::CHAIN_GUEST_ELF
}

pub(crate) const API_VERSION: &str = "1.2.3";

#[derive(new)]
pub(crate) struct Context {
    client: Client,
    anvil: Anvil,
    chain_proof_server: ChainProofServer,
    gas_meter_server: Option<GasMeterServer>,
}

impl Context {
    pub(crate) async fn default() -> Self {
        let anvil = Anvil::start();
        let client = anvil.setup_client();
        let chain_proof_server =
            ChainProofServer::start(CHAIN_PROOF_POLL_INTERVAL, CHAIN_PROOF_TIMEOUT).await;
        let mut ctx = Self::new(client, anvil, chain_proof_server, None);
        ctx.mock_latest_block().await;
        ctx
    }

    pub(crate) fn with_gas_meter_server(mut self, gas_meter_server: GasMeterServer) -> Self {
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

    pub(crate) fn server(&self, call_guest_elf: GuestElf, chain_guest_elf: &GuestElf) -> Server {
        let gas_meter_config = self
            .gas_meter_server
            .as_ref()
            .map(GasMeterServer::as_gas_meter_config);
        let chain_proof_config = self.chain_proof_server.as_chain_proof_config();
        let chain_guest_ids = vec![chain_guest_elf.id].into_boxed_slice();
        let config = ConfigBuilder::new(call_guest_elf, chain_guest_ids, API_VERSION.into())
            .with_chain_proof_config(chain_proof_config)
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
    use call_server_lib::{
        chain_proof::Config as ChainProofConfig, gas_meter::Config as GasMeterConfig, server,
        Config,
    };
    use derive_more::{Deref, DerefMut};
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
    use mock_chain_server::ChainProofServerMock;
    use provider::to_eth_block_header;
    use serde::Serialize;
    use server_utils::{post, post_with_bearer_auth, rpc::mock::Server as RpcServerMock};

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

        pub(crate) async fn post_with_bearer_auth(
            &self,
            url: &str,
            body: impl Serialize,
            token: &str,
        ) -> Response<Body> {
            post_with_bearer_auth(self.0.clone(), url, &body, token).await
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

    #[derive(Deref, DerefMut)]
    pub(crate) struct GasMeterServer {
        #[deref]
        #[deref_mut]
        mock: RpcServerMock,
        time_to_live: Duration,
        api_key: Option<String>,
    }

    impl GasMeterServer {
        pub(crate) async fn start(time_to_live: Duration, api_key: Option<String>) -> Self {
            let mock = RpcServerMock::start().await;
            Self {
                mock,
                time_to_live,
                api_key,
            }
        }

        pub(crate) fn as_gas_meter_config(&self) -> GasMeterConfig {
            GasMeterConfig::new(self.url(), self.time_to_live, self.api_key.clone())
        }
    }

    #[derive(Deref, DerefMut)]
    pub(crate) struct ChainProofServer {
        #[deref]
        #[deref_mut]
        mock: ChainProofServerMock,
        poll_interval: Duration,
        timeout: Duration,
    }

    impl ChainProofServer {
        pub(crate) async fn start(poll_interval: Duration, timeout: Duration) -> Self {
            let mock = ChainProofServerMock::start().await;
            Self {
                mock,
                poll_interval,
                timeout,
            }
        }

        pub(crate) fn as_chain_proof_config(&self) -> ChainProofConfig {
            ChainProofConfig::new(self.url(), self.poll_interval, self.timeout)
        }
    }
}
