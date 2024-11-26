use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::{RpcClient, RpcError};

use crate::handlers::v_call::types::CallHash;

#[derive(new, Serialize, Debug)]
#[serde(deny_unknown_fields)]
struct AllocateGas {
    hash: CallHash,
    gas_limit: u64,
    time_to_live: u64,
}

#[derive(Serialize, Debug)]
#[allow(unused)]
pub enum ComputationStage {
    Preflight,
    Proving,
}

#[derive(Serialize, Debug)]
#[serde(deny_unknown_fields)]
#[allow(unused)]
struct RefundUnusedGas {
    hash: CallHash,
    computation_stage: ComputationStage,
    gas_used: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub time_to_live: u64,
}

pub struct Client {
    client: RpcClient,
    hash: CallHash,
    time_to_live: u64,
}

pub const V_ALLOCATE_GAS: &str = "v_allocateGas";

impl Client {
    pub fn new(url: &str, hash: CallHash, time_to_live: u64) -> Self {
        let client = RpcClient::new(url, V_ALLOCATE_GAS);
        Self {
            client,
            hash,
            time_to_live,
        }
    }

    pub async fn allocate_gas(&self, gas_limit: u64) -> Result<(), RpcError> {
        let req = AllocateGas::new(self.hash, gas_limit, self.time_to_live);
        let _resp = self.client.call(&req).await?;
        Ok(())
    }
}
