use std::time::Duration;

use async_trait::async_trait;
use auto_impl::auto_impl;
use call_common::Metadata;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::rpc::{Client as RawRpcClient, Error as RpcError, Method};
use tracing::{error, info};

use crate::{handlers::v_call::types::CallHash, token::Token};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Rpc(#[from] RpcError),
}

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

#[derive(new, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct SendMetadata {
    hash: CallHash,
    metadata: Box<[Metadata]>,
}

impl Method for SendMetadata {
    const METHOD_NAME: &str = "v_sendMetadata";
}

#[derive(new, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub time_to_live: Duration,
    pub api_key: Option<String>,
}

#[async_trait]
#[auto_impl(Box)]
pub trait Client: Send + Sync {
    async fn allocate(&self, gas_limit: u64) -> Result<()>;
    async fn refund(&self, stage: ComputationStage, gas_used: u64) -> Result<()>;
    async fn send_metadata(&self, metadata: Box<[Metadata]>) -> Result<()>;
}

pub struct RpcClient {
    client: RawRpcClient,
    hash: CallHash,
    time_to_live: Duration,
    api_key: Option<String>,
    token: Option<Token>,
}

impl RpcClient {
    const PROVER_API_KEY_HEADER_NAME: &str = "x-prover-api-key";

    pub fn new(
        Config {
            url,
            time_to_live,
            api_key,
        }: Config,
        hash: CallHash,
        token: Option<Token>,
    ) -> Self {
        let client = RawRpcClient::new(&url);
        Self {
            client,
            hash,
            time_to_live,
            api_key,
            token,
        }
    }

    async fn call(&self, method: impl Method) -> Result<()> {
        let mut req = self.client.request(method);
        if let Some(api_key) = &self.api_key {
            req = req.with_header(Self::PROVER_API_KEY_HEADER_NAME, api_key);
        }
        if let Some(token) = &self.token {
            req = req.with_bearer_auth(token);
        }
        let _resp = req.send().await?;
        Ok(())
    }
}

#[async_trait]
impl Client for RpcClient {
    async fn allocate(&self, gas_limit: u64) -> Result<()> {
        let req = AllocateGas::new(self.hash, gas_limit, self.time_to_live.as_secs());
        info!("v_allocateGas => {req:#?}");
        if let Err(err) = self.call(req).await {
            error!("v_allocateGas failed with error: {err}");
            return Err(err);
        }
        Ok(())
    }

    async fn refund(&self, stage: ComputationStage, gas_used: u64) -> Result<()> {
        let req = RefundUnusedGas::new(self.hash, stage, gas_used);
        info!("v_refundUnusedGas => {req:#?}");
        if let Err(err) = self.call(req).await {
            error!("v_refundGas failed with error: {err}");
            return Err(err);
        }
        Ok(())
    }

    async fn send_metadata(&self, metadata: Box<[Metadata]>) -> Result<()> {
        let req = SendMetadata::new(self.hash, metadata);
        info!("v_sendMetadata => {req:#?}");
        if let Err(err) = self.call(req).await {
            error!("v_sendMetadata failed with error: {err}");
            return Err(err);
        }
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

    async fn send_metadata(&self, _metadata: Box<[Metadata]>) -> Result<()> {
        Ok(())
    }
}

pub fn init(config: Option<Config>, call_hash: CallHash, token: Option<Token>) -> Box<dyn Client> {
    config.map_or(Box::new(NoOpClient), |config| {
        Box::new(RpcClient::new(config, call_hash, token))
    })
}
