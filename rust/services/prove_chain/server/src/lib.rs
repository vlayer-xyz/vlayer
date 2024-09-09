use std::pin::Pin;

use axum::{routing::post, Router};
use axum_jrpc::{
    error::{JsonRpcError, JsonRpcErrorReason},
    Value,
};
use serde::{Deserialize, Serialize};
use server_utils::route;
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct Params {}

#[derive(Serialize)]
pub struct ChainProof;

#[derive(Debug, Error)]
pub enum Error {}

impl From<Error> for JsonRpcError {
    fn from(error: Error) -> Self {
        JsonRpcError::new(
            JsonRpcErrorReason::InternalError,
            error.to_string(),
            Value::Null,
        )
    }
}

async fn v_prove_chain(_params: Params) -> Result<ChainProof, Error> {
    Ok(ChainProof)
}

pub fn server() -> Router {
    let v_prove_chain_handler = move |params| Box::pin(v_prove_chain(params)) as Pin<Box<_>>;
    let jrpc_handler =
        move |req| async move { route(req, "v_proveChain", v_prove_chain_handler).await };

    Router::new().route("/", post(jrpc_handler))
}
