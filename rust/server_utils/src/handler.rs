use std::future::Future;

use axum_jrpc::error::JsonRpcError;
use serde::{de::DeserializeOwned, Serialize};

pub trait JsonRpcHandler {
    type Params: DeserializeOwned;
    type Result: Serialize;
    fn call(
        &self,
        params: Self::Params,
    ) -> impl Future<Output = Result<Self::Result, impl Into<JsonRpcError>>> + Send;
}
