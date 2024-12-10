use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::{Client as RawRpcClient, Method, Result};

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
#[serde(rename_all = "snake_case")]
pub enum ComputationStage {
    Preflight,
    Proving,
}

#[derive(new, Serialize, Debug)]
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

#[async_trait]
#[auto_impl(Box)]
pub trait Client: Send + Sync {
    async fn allocate(&self, gas_limit: u64) -> Result<()>;
    async fn refund(&self, stage: ComputationStage, gas_used: u64) -> Result<()>;
}

pub struct RpcClient {
    client: RawRpcClient,
    hash: CallHash,
    time_to_live: u64,
}

impl RpcClient {
    pub fn new(Config { url, time_to_live }: Config, hash: CallHash) -> Self {
        let client = RawRpcClient::new(&url);
        Self {
            client,
            hash,
            time_to_live,
        }
    }
}

#[async_trait]
impl Client for RpcClient {
    async fn allocate(&self, gas_limit: u64) -> Result<()> {
        let req = AllocateGas::new(self.hash, gas_limit, self.time_to_live);
        let _resp = self.client.call(req).await?;
        Ok(())
    }

    async fn refund(&self, stage: ComputationStage, gas_used: u64) -> Result<()> {
        let req = RefundUnusedGas::new(self.hash, stage, gas_used);
        let _resp = self.client.call(req).await?;
        Ok(())
    }
}

pub struct NoOpClient;

#[async_trait]
impl Client for NoOpClient {
    async fn allocate(&self, _gas_limit: u64) -> Result<()> {
        Ok(())
    }

    async fn refund(&self, _stage: ComputationStage, _gas_used: u64) -> Result<()> {
        Ok(())
    }
}
