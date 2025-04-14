use axum::{body::Bytes, extract::State, response::IntoResponse, routing::post};
use chain_db::ChainDb;
use server_utils::{RequestIdLayer, Router as JrpcRouter, cors, init_trace_layer};
use tokio::net::TcpListener;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::info;

use crate::{
    ServerConfig,
    handlers::{RpcServer, State as AppState},
};

async fn handle_jrpc(State(router): State<JrpcRouter<AppState>>, body: Bytes) -> impl IntoResponse {
    router.handle_request(body).await
}

pub fn server(chain_db: ChainDb) -> axum::Router {
    let router = JrpcRouter::new(AppState::new(chain_db).into_rpc());
    axum::Router::new()
        .route("/", post(handle_jrpc))
        .with_state(router)
        .layer(cors())
        .layer(ValidateRequestHeaderLayer::accept(mime::APPLICATION_JSON.as_ref()))
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
