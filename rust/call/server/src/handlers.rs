use axum_jrpc::error::JsonRpcError;
use serde::{de::DeserializeOwned, Serialize};

pub mod v_call;

pub trait JsonRpcHandler {
    type Params: DeserializeOwned;
    type Result: Serialize;
    async fn call(&self, params: Self::Params) -> Result<Self::Result, impl Into<JsonRpcError>>;
}
