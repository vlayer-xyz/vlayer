use std::{future::Future, pin::Pin, sync::Arc};

use axum::{extract::State, response::IntoResponse, routing::post};
use axum_jrpc::JsonRpcExtractor;
use chain_db::ChainDb;
use parking_lot::RwLock;
use server_utils::{init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    handlers::{status::v_sync_status, v_chain::v_chain},
    ServerConfig,
};

async fn handle_jrpc(
    State(router): State<server_utils::Router>,
    request: JsonRpcExtractor,
) -> impl IntoResponse {
    router.handle_request(request).await
}

pub fn server(chain_db: ChainDb) -> axum::Router {
    let chain_db = Arc::new(RwLock::new(chain_db));
    let chain_db_ = chain_db.clone();
    let mut jrpc_router = server_utils::Router::default();
    jrpc_router.add_handler("v_chain", move |params| -> Pin<Box<dyn Future<Output = _> + Send>> {
        let chain_db = chain_db.clone();
        Box::pin(v_chain(chain_db, params))
    });
    jrpc_router.add_handler(
        "v_sync_status",
        move |params| -> Pin<Box<dyn Future<Output = _> + Send>> {
            let chain_db = chain_db_.clone();
            Box::pin(v_sync_status(chain_db, params))
        },
    );
    axum::Router::new()
        .route("/", post(handle_jrpc))
        .with_state(jrpc_router)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}

pub async fn serve(config: ServerConfig, db: ChainDb) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.listen_addr).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(db)).await?;

    Ok(())
}
