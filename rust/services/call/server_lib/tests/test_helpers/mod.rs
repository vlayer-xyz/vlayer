use std::time::Duration;

use blockchain::{Anvil, Client, Contract};
use call_server_lib::{ConfigBuilder, ProofMode};
use common::GuestElf;
use derive_new::new;
use ethers::types::{Bytes, H160};
use serde_json::{json, Value};

pub(crate) mod blockchain;
pub(crate) mod call;
pub(crate) mod gas_meter;

pub const GAS_LIMIT: u64 = 1_000_000;
pub const ETHEREUM_SEPOLIA_ID: u64 = 11_155_111;
pub const GAS_METER_TTL: Duration = Duration::from_secs(3600);

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
    gas_meter_server: Option<gas_meter::Server>,
}

impl Context {
    pub(crate) fn default() -> Self {
        let anvil = Anvil::start();
        let client = anvil.setup_client();
        Self::new(client, anvil, None)
    }

    pub(crate) fn with_gas_meter_server(mut self, gas_meter_server: gas_meter::Server) -> Self {
        self.gas_meter_server = Some(gas_meter_server);
        self
    }

    pub(crate) fn assert_gas_meter(&self) {
        self.gas_meter_server
            .as_ref()
            .expect("gas meter server not set up")
            .assert();
    }

    pub(crate) async fn deploy_contract(&self) -> Contract {
        self.client.deploy_contract().await
    }

    pub(crate) fn server(
        &self,
        call_guest_elf: GuestElf,
        chain_guest_elf: &GuestElf,
    ) -> call::Server {
        let gas_meter_config = self
            .gas_meter_server
            .as_ref()
            .map(gas_meter::Server::as_gas_meter_config);
        let chain_guest_ids = vec![chain_guest_elf.id].into_boxed_slice();
        let config = ConfigBuilder::new(call_guest_elf, chain_guest_ids, API_VERSION.into())
            .with_rpc_mappings([(self.anvil.chain_id(), self.anvil.endpoint())])
            .with_proof_mode(ProofMode::Fake)
            .with_gas_meter_config(gas_meter_config)
            .build();
        call::Server::new(config)
    }
}
