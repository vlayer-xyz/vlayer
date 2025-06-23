use std::time::Duration;

use async_trait::async_trait;
use auto_impl::auto_impl;
use call_common::Metadata;
use derive_more::Debug;
use derive_new::new;
use serde::{Deserialize, Serialize};
use server_utils::{
    jwt::axum::Token,
    rpc::{Client as RawRpcClient, Error as RpcError, Method},
};
use strum::{Display, EnumString, VariantNames};
use tracing::{error, info};

use crate::handlers::v_call::types::CallHash;

type Result<T> = std::result::Result<T, Error>;

pub const PROVER_API_KEY_HEADER_NAME: &str = "x-prover-api-key";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Rpc(#[from] RpcError),
    #[error("Token is required for billing mode")]
    TokenRequired,
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

#[derive(new, Serialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UpdateCycles {
    hash: CallHash,
    cycles_used: u64,
}

impl Method for UpdateCycles {
    const METHOD_NAME: &str = "v_updateCycles";
}

#[derive(new, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub url: String,
    pub time_to_live: Duration,
    #[debug(skip)]
    pub api_key: String,
    pub mode: Mode,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default, Display, EnumString, VariantNames)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum Mode {
    #[default]
    Bill,
    Track,
}

#[async_trait]
#[auto_impl(Box)]
pub trait Client: Send + Sync {
    async fn allocate(&self, gas_limit: u64) -> Result<()>;
    async fn refund(&self, stage: ComputationStage, gas_used: u64) -> Result<()>;
    async fn send_metadata(&self, metadata: Box<[Metadata]>) -> Result<()>;
    async fn update_cycles(&self, cycles_used: u64) -> Result<()>;
}

pub struct BillingClient {
    client: RawRpcClient,
    hash: CallHash,
    time_to_live: Duration,
    api_key: String,
    token: Token,
}

pub struct TrackingClient {
    client: RawRpcClient,
    hash: CallHash,
    api_key: String,
}

pub struct NoOpClient;

impl BillingClient {
    pub fn new(
        Config {
            url,
            time_to_live,
            api_key,
            ..
        }: Config,
        hash: CallHash,
        token: Token,
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
        self.client
            .request(method)
            .with_header(PROVER_API_KEY_HEADER_NAME, &self.api_key)
            .with_bearer_auth(&self.token)
            .send()
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Client for BillingClient {
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

    async fn update_cycles(&self, cycles_used: u64) -> Result<()> {
        let req = UpdateCycles::new(self.hash, cycles_used);
        info!("v_updateCycles => {req:#?}");
        if let Err(err) = self.call(req).await {
            error!("v_updateCycles failed with error: {err}");
            return Err(err);
        }
        Ok(())
    }
}

impl TrackingClient {
    pub fn new(Config { url, api_key, .. }: Config, hash: CallHash) -> Self {
        let client = RawRpcClient::new(&url);
        Self {
            client,
            hash,
            api_key,
        }
    }

    async fn call(&self, method: impl Method) -> Result<()> {
        self.client
            .request(method)
            .with_header(PROVER_API_KEY_HEADER_NAME, &self.api_key)
            .send()
            .await?;
        Ok(())
    }
}

#[async_trait]
impl Client for TrackingClient {
    async fn allocate(&self, _gas_limit: u64) -> Result<()> {
        Ok(())
    }

    async fn refund(&self, _stage: ComputationStage, _gas_used: u64) -> Result<()> {
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

    async fn update_cycles(&self, cycles_used: u64) -> Result<()> {
        let req = UpdateCycles::new(self.hash, cycles_used);
        info!("v_updateCycles => {req:#?}");
        if let Err(err) = self.call(req).await {
            error!("v_updateCycles failed with error: {err}");
            return Err(err);
        }
        Ok(())
    }
}

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

    async fn update_cycles(&self, _cycles_used: u64) -> Result<()> {
        Ok(())
    }
}

pub fn try_init(
    config: Option<Config>,
    call_hash: CallHash,
    token: Option<Token>,
) -> Result<Box<dyn Client>> {
    let Some(config) = config else {
        return Ok(Box::new(NoOpClient));
    };
    Ok(match config.mode {
        Mode::Bill => {
            let token = token.ok_or(Error::TokenRequired)?;
            Box::new(BillingClient::new(config, call_hash, token))
        }
        Mode::Track => Box::new(TrackingClient::new(config, call_hash)),
    })
}
