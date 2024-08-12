use async_trait::async_trait;
use axum_jrpc::{JrpcResult, JsonRpcExtractor};

#[async_trait]
pub trait JsonRpcHandler {
    async fn call(&self, request: JsonRpcExtractor) -> JrpcResult;
}

pub mod v_call;
