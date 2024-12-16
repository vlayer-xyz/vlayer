use std::sync::Arc;

use async_trait::async_trait;
use call_engine::HostOutput;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use jsonrpsee::proc_macros::rpc;
use v_call::types::{Call, CallContext, CallHash};
use v_get_proof_receipt::types::CallResult;
use v_versions::Versions;

use crate::{config::Config, error::AppError};

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

#[rpc(server)]
#[async_trait]
pub trait Rpc {
    #[method(name = "v_call")]
    async fn v_call(&self, call: Call, ctx: CallContext) -> Result<CallHash, AppError>;

    #[method(name = "v_getProofReceipt")]
    async fn v_get_proof_receipt(&self, hash: CallHash) -> Result<CallResult, AppError>;

    #[method(name = "v_versions")]
    async fn v_versions(&self) -> Result<Versions, AppError>;
}

#[derive(Deref, DerefMut, Default, Debug)]
pub struct Proofs(DashMap<CallHash, ProofStatus>);

#[allow(dead_code)] // To be used in the future
#[derive(Debug)]
pub enum ProofStatus {
    WaitingForChainProof,
    Preflight,
    Proving,
    Ready(Result<HostOutput, AppError>),
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
    async fn v_call(&self, call: Call, ctx: CallContext) -> Result<CallHash, AppError> {
        v_call::v_call(self.config.clone(), self.proofs.clone(), call, ctx).await
    }

    async fn v_get_proof_receipt(&self, hash: CallHash) -> Result<CallResult, AppError> {
        v_get_proof_receipt::v_get_proof_receipt(&self.proofs, hash)
    }

    async fn v_versions(&self) -> Result<Versions, AppError> {
        v_versions::v_versions(&self.config)
    }
}
