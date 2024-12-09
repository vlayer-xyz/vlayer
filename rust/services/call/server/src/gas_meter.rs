use std::convert::From;

use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::{Client as RawRpcClient, Method, Result};

use crate::{config::Config as ServerConfig, handlers::v_call::types::CallHash};

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

impl From<&ServerConfig> for Box<dyn Client> {
    fn from(value: &ServerConfig) -> Self {
        match value.gas_meter_config() {
            Some(Config { url, time_to_live }) => Box::new(RpcClient::new(&url, time_to_live)),
            None => Box::new(NoOpClient),
        }
    }
}

#[async_trait]
pub trait Client: Send {
    async fn allocate(&self, hash: CallHash, gas_limit: u64) -> Result<()>;
    async fn refund(&self, hash: CallHash, stage: ComputationStage, gas_used: u64) -> Result<()>;
}

pub struct RpcClient {
    client: RawRpcClient,
    time_to_live: u64,
}

impl RpcClient {
    pub fn new(url: &str, time_to_live: u64) -> Self {
        let client = RawRpcClient::new(url);
        Self {
            client,
            time_to_live,
        }
    }
}

#[async_trait]
impl Client for RpcClient {
    async fn allocate(&self, hash: CallHash, gas_limit: u64) -> Result<()> {
        let req = AllocateGas::new(hash, gas_limit, self.time_to_live);
        let _resp = self.client.call(req).await?;
        Ok(())
    }

    async fn refund(&self, hash: CallHash, stage: ComputationStage, gas_used: u64) -> Result<()> {
        let req = RefundUnusedGas::new(hash, stage, gas_used);
        let _resp = self.client.call(req).await?;
        Ok(())
    }
}

pub struct NoOpClient;

#[async_trait]
impl Client for NoOpClient {
    async fn allocate(&self, _hash: CallHash, _gas_limit: u64) -> Result<()> {
        Ok(())
    }

    async fn refund(
        &self,
        _hash: CallHash,
        _stage: ComputationStage,
        _gas_used: u64,
    ) -> Result<()> {
        Ok(())
    }
}
