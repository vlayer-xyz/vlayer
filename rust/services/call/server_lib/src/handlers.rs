use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use jsonrpsee::{proc_macros::rpc, Extensions};
use serde::Deserialize;
use v_call::types::{Call, CallContext, CallHash, Result as VCallResult};
use v_get_proof_receipt::types::{CallResult, Result as VGetProofReceiptResult};
use v_versions::Versions;

use crate::{
    config::Config,
    metrics::Metrics,
    proof::{Error as ProofError, RawData},
};

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

pub type UserToken = String;

#[derive(Clone, Debug, Deserialize)]
pub struct QueryParams {
    token: Option<UserToken>,
}

#[rpc(server)]
#[async_trait]
pub trait Rpc {
    #[method(name = "v_call", with_extensions)]
    async fn v_call(&self, call: Call, ctx: CallContext) -> VCallResult<CallHash>;

    #[method(name = "v_getProofReceipt")]
    async fn v_get_proof_receipt(&self, hash: CallHash) -> VGetProofReceiptResult<CallResult>;

    #[method(name = "v_versions")]
    async fn v_versions(&self) -> Versions;
}

#[derive(Deref, DerefMut, Default)]
pub struct Proofs(DashMap<CallHash, ProofStatus>);

pub enum ProofStatus {
    /// Proof task has just been queued
    Queued,
    /// Waiting for chain service to generate proof for the start execution location
    ChainProof,
    ChainProofError(ProofError),
    /// Preflight computation in progress
    Preflight,
    PreflightError((Metrics, ProofError)),
    /// Proof is being generated
    Proving,
    ProvingError((Metrics, ProofError)),
    /// Proof generation finished
    Done((Metrics, RawData)),
}

impl ProofStatus {
    pub fn is_err(&self) -> bool {
        match self {
            Self::ChainProofError(..) | Self::PreflightError(..) | Self::ProvingError(..) => true,
            _ => false,
        }
    }

    pub fn metrics(&self) -> Metrics {
        match self {
            Self::PreflightError((metrics, ..))
            | Self::ProvingError((metrics, ..))
            | Self::Done((metrics, ..)) => *metrics,
            _ => Metrics::default(),
        }
    }

    pub fn data(&self) -> Option<RawData> {
        match self {
            Self::Done((_, data)) => Some(data.clone()),
            _ => None,
        }
    }

    pub fn err(&self) -> Option<&ProofError> {
        match self {
            Self::ChainProofError(err)
            | Self::PreflightError((_, err))
            | Self::ProvingError((_, err)) => Some(err),
            _ => None,
        }
    }
}

pub type SharedConfig = Arc<Config>;
pub type SharedProofs = Arc<Proofs>;

#[derive(Clone)]
pub struct State {
    config: Arc<Config>,
    proofs: Arc<Proofs>,
}

impl State {
    pub fn new(cfg: Config) -> Self {
        let config = Arc::new(cfg);
        let proofs = Arc::new(Proofs::default());
        Self { config, proofs }
    }
}

#[async_trait]
impl RpcServer for State {
    async fn v_call(
        &self,
        extensions: &Extensions,
        call: Call,
        ctx: CallContext,
    ) -> VCallResult<CallHash> {
        let params = extensions
            .get::<QueryParams>()
            .expect("query params should be extracted in the handler");
        v_call::v_call(self.config.clone(), self.proofs.clone(), params.clone(), call, ctx).await
    }

    async fn v_get_proof_receipt(&self, hash: CallHash) -> VGetProofReceiptResult<CallResult> {
        v_get_proof_receipt::v_get_proof_receipt(&self.proofs, hash)
    }

    async fn v_versions(&self) -> Versions {
        v_versions::v_versions(&self.config)
    }
}
