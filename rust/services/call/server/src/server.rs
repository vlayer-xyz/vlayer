use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use jsonrpsee::{types::Request, ConnectionId, MethodCallback, RpcModule};
use server_utils::{init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    config::Config,
    handlers::{RpcServer, SharedState, State2},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

async fn handle_request(State(state): State<RpcModule<State2>>, body: Bytes) -> impl IntoResponse {
    let request: Request = serde_json::from_slice(&body)?;
    let exts = state.extensions().clone();
    let conn_id = ConnectionId(0);
    match state.method(request.method_name())? {
        MethodCallback::Async(cb) => {
            let response = cb(
                request.id().into_owned(),
                request.params().into_owned(),
                conn_id,
                usize::MAX,
                exts,
            )
            .await;
            Response::new(response.into_result())
        }
        _ => todo!("implement other method types in handler"),
    }
}

pub fn server(cfg: Config) -> Router {
    let config = Arc::new(cfg);
    let state = SharedState::default();
    let full_state = State2::new(config, state).into_rpc();

    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    let cors = CorsLayer::permissive();
    Router::new()
        .route("/", post(handle_request))
        .with_state(full_state)
        .layer(cors)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
