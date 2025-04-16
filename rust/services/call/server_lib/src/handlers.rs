use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use derive_new::new;
use jsonrpsee::{Extensions, proc_macros::rpc};
use server_utils::RequestId;
use v_call::types::{Call, CallContext, CallHash, Result as VCallResult};
use v_get_proof_receipt::types::{CallResult, Result as VGetProofReceiptResult};
use v_versions::Versions;

use crate::{config::Config, proof::Status as ProofStatus, token::Token};

pub mod v_call;
pub mod v_get_proof_receipt;
pub mod v_versions;

#[derive(new, Clone)]
pub struct Params {
    pub config: Config,
    pub token: Option<Token>,
    pub req_id: RequestId,
}

#[rpc(server)]
#[async_trait]
pub trait Rpc {
    #[method(name = "v_call", with_extensions)]
    async fn v_call(&self, call: Call, ctx: CallContext) -> VCallResult<CallHash>;

    #[method(name = "v_getProofReceipt")]
    async fn v_get_proof_receipt(&self, hash: CallHash) -> VGetProofReceiptResult<CallResult>;

    #[method(name = "v_versions", with_extensions)]
    async fn v_versions(&self) -> Versions;
}

#[derive(Deref, DerefMut, Default)]
pub struct Proofs(DashMap<CallHash, ProofStatus>);

pub type State = Arc<Proofs>;

#[async_trait]
#[allow(clippy::expect_used)]
impl RpcServer for State {
    async fn v_call(
        &self,
        extensions: &Extensions,
        call: Call,
        ctx: CallContext,
    ) -> VCallResult<CallHash> {
        let params = extensions
            .get::<Params>()
            .expect("params should be extracted in the handler");
        v_call::v_call(self.clone(), call, ctx, params.clone()).await
    }

    async fn v_get_proof_receipt(&self, hash: CallHash) -> VGetProofReceiptResult<CallResult> {
        v_get_proof_receipt::v_get_proof_receipt(self, hash)
    }

    async fn v_versions(&self, extensions: &Extensions) -> Versions {
        let params = extensions
            .get::<Params>()
            .expect("params should be extracted in the handler");
        v_versions::v_versions(&params.config)
    }
}
