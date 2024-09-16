use std::{pin::Pin, sync::Arc};

use axum::{routing::post, Router};
use mpt::MerkleTrie;
use server_utils::route;

use crate::{config::ServerConfig, handlers::v_prove_chain::v_prove_chain};

pub fn server(config: ServerConfig) -> Router {
    let config = Arc::new(config);
    let call_and_convert_to_json = |config: Arc<ServerConfig>, params| async move {
        v_prove_chain(config.clone(), MerkleTrie::default(), params)
            .await
            .map(|x| x.to_json())
    };
    let v_prove_chain_handler =
        move |params| Box::pin(call_and_convert_to_json(config.clone(), params)) as Pin<Box<_>>;
    let jrpc_handler =
        move |req| async move { route(req, "v_proveChain", v_prove_chain_handler).await };

    Router::new().route("/", post(jrpc_handler))
}
