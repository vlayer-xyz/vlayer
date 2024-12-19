use async_trait::async_trait;
use auto_impl::auto_impl;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::{Client as RawRpcClient, Method, Result};
use tracing::info;

use crate::handlers::{v_call::types::CallHash, UserToken};

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
    pub api_key: Option<String>,
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
    api_key: Option<String>,
    user_token: Option<UserToken>,
}

impl RpcClient {
    const API_KEY_HEADER_NAME: &str = "x-prover-api-key";
    const USER_TOKEN_QUERY_KEY: &str = "key";

    pub fn new(
        Config {
            url,
            time_to_live,
            api_key,
        }: Config,
        hash: CallHash,
        user_token: Option<UserToken>,
    ) -> Self {
        let client = RawRpcClient::new(&url);
        Self {
            client,
            hash,
            time_to_live,
            api_key,
            user_token,
        }
    }

    async fn call(&self, method: impl Method) -> Result<()> {
        let mut req = self.client.request(method);
        if let Some(api_key) = &self.api_key {
            req = req.with_header(Self::API_KEY_HEADER_NAME, api_key);
        }
        if let Some(user_token) = &self.user_token {
            req = req.with_query(Self::USER_TOKEN_QUERY_KEY, user_token);
        }
        let _resp = req.send().await?;
        Ok(())
    }
}

#[async_trait]
impl Client for RpcClient {
    async fn allocate(&self, gas_limit: u64) -> Result<()> {
        let req = AllocateGas::new(self.hash, gas_limit, self.time_to_live);
        info!("v_allocateGas => {req:#?}");
        self.call(req).await
    }

    async fn refund(&self, stage: ComputationStage, gas_used: u64) -> Result<()> {
        let req = RefundUnusedGas::new(self.hash, stage, gas_used);
        info!("v_refundUnusedGas => {req:#?}");
        self.call(req).await
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
