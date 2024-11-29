use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::{Client as RpcClient, Method, Result};

use crate::handlers::v_call::types::CallHash;

#[derive(new, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AllocateGas {
    hash: CallHash,
    gas_limit: u64,
    time_to_live: u64,
}

impl Method for AllocateGas {
    const METHOD_NAME: &str = "v_allocateGas";
}

#[derive(Serialize, Debug)]
pub enum ComputationStage {
    Preflight,
    Proving,
}

#[derive(Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RefundUnusedGas {
    hash: CallHash,
    computation_stage: ComputationStage,
    gas_used: u64,
}

impl Method for RefundUnusedGas {
    const METHOD_NAME: &str = "v_refundUnusedGas";
}

#[derive(new, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub time_to_live: u64,
}

pub struct Client {
    client: RpcClient,
    hash: CallHash,
    time_to_live: u64,
}

impl Client {
    pub fn new(url: &str, hash: CallHash, time_to_live: u64) -> Self {
        let client = RpcClient::new(url);
        Self {
            client,
            hash,
            time_to_live,
        }
    }

    pub async fn allocate_gas(&self, gas_limit: u64) -> Result<()> {
        let req = AllocateGas::new(self.hash, gas_limit, self.time_to_live);
        let _resp = self.client.call(req).await?;
        Ok(())
    }
}
