use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    body::Bytes,
    http::{header::CONTENT_TYPE, status::StatusCode},
    response::IntoResponse,
};
use call_engine::HostOutput;
use dashmap::DashMap;
use derive_more::{Deref, DerefMut};
use derive_new::new;
use jsonrpsee::{
    proc_macros::rpc, types::Request, ConnectionId, MethodCallback, MethodResponse, RpcModule,
};
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
pub struct Proofs(DashMap<CallHash, Result<HostOutput, AppError>>);

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

#[derive(new, Clone)]
pub struct Router<T: Send + Sync + Clone + 'static>(RpcModule<T>);

impl<T> Router<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub async fn handle_request(self, body: Bytes) -> impl IntoResponse {
        match serde_json::from_slice::<Request>(&body) {
            Ok(request) => {
                let response = self.handle_request_inner(request).await;
                (StatusCode::OK, [(CONTENT_TYPE, "appication/json")], response.to_result())
                    .into_response()
            }
            Err(..) => StatusCode::BAD_REQUEST.into_response(),
        }
    }

    async fn handle_request_inner(mut self, request: Request<'_>) -> MethodResponse {
        let id = request.id().into_owned();
        let params = request.params().into_owned();
        let exts = self.0.extensions().clone();
        let conn_id = ConnectionId(0);
        if let Some(method) = self.0.method(request.method_name()) {
            match method {
                MethodCallback::Async(cb) => cb(id, params, conn_id, usize::MAX, exts).await,
                _ => todo!("implement other method types in handler"),
            }
        } else {
            let err = AppError::MethodNotFound(request.method_name().to_string());
            MethodResponse::error(id, err)
        }
    }
}
