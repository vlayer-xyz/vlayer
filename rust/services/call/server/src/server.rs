use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::State,
    http::{header::CONTENT_TYPE, status::StatusCode},
    response::IntoResponse,
    routing::post,
    Router,
};
use jsonrpsee::{types::Request, ConnectionId, MethodCallback, MethodResponse, RpcModule};
use server_utils::{init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    config::Config,
    error::AppError,
    handlers::{RpcServer, SharedState, State2},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

async fn handle_request(mut state: RpcModule<State2>, request: Request<'_>) -> MethodResponse {
    let id = request.id().into_owned();
    let params = request.params().into_owned();
    let exts = state.extensions().clone();
    let conn_id = ConnectionId(0);
    if let Some(method) = state.method(request.method_name()) {
        match method {
            MethodCallback::Async(cb) => cb(id, params, conn_id, usize::MAX, exts).await,
            _ => todo!("implement other method types in handler"),
        }
    } else {
        let err = AppError::MethodNotFound(request.method_name().to_string());
        MethodResponse::error(id, err)
    }
}

async fn handle(State(state): State<RpcModule<State2>>, body: Bytes) -> impl IntoResponse {
    match serde_json::from_slice::<Request>(&body) {
        Ok(request) => {
            let response = handle_request(state, request).await;
            (StatusCode::OK, [(CONTENT_TYPE, "appication/json")], response.to_result())
                .into_response()
        }
        Err(..) => StatusCode::BAD_REQUEST.into_response(),
    }
}

pub fn server(cfg: Config) -> Router {
    let config = Arc::new(cfg);
    let state = SharedState::default();
    let full_state = State2::new(config, state).into_rpc();

    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    let cors = CorsLayer::permissive();
    Router::new()
        .route("/", post(handle))
        .with_state(full_state)
        .layer(cors)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
