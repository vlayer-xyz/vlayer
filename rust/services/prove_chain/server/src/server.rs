use std::pin::Pin;

use axum::{routing::post, Router};
use server_utils::route;

use crate::handlers::v_prove_chain::v_prove_chain;

pub fn server() -> Router {
    let v_prove_chain_handler = move |params| Box::pin(v_prove_chain(params)) as Pin<Box<_>>;
    let jrpc_handler =
        move |req| async move { route(req, "v_proveChain", v_prove_chain_handler).await };

    Router::new().route("/", post(jrpc_handler))
}
