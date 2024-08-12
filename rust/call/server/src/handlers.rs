use async_trait::async_trait;
use axum_jrpc::{JrpcResult, JsonRpcExtractor};

#[async_trait]
pub trait JsonRpcHandler {
    type Config;
    async fn call(&self, request: JsonRpcExtractor, config: Self::Config) -> JrpcResult;
}

pub mod v_call;
