use std::pin::Pin;

use axum::{routing::post, Router};
use mpt::MerkleTrie;
use server_utils::route;

use crate::handlers::v_chain::v_chain;
pub use crate::{handlers::v_chain::ChainProof, mock::ChainProofServerMock};

pub fn server() -> Router {
    let v_chain_handler =
        move |params| Box::pin(v_chain(MerkleTrie::default(), params)) as Pin<Box<_>>;
    let jrpc_handler = move |req| async move { route(req, "v_chain", v_chain_handler).await };

    Router::new().route("/", post(jrpc_handler))
}
