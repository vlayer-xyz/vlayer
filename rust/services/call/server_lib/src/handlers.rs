use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use jsonrpsee::{proc_macros::rpc, Extensions};
use serde::Deserialize;
use v_call::types::{Call, CallContext, CallHash, Result as VCallResult};
use v_get_proof_receipt::types::{CallResult, Result as VGetProofReceiptResult};
use v_versions::Versions;

use crate::{config::Config, proof::ProofStatus};

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
