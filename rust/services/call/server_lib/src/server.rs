use axum::{
    body::Bytes,
    extract::{Query, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use server_utils::{cors, init_trace_layer, RequestIdLayer, Router as JrpcRouter};
use tokio::net::TcpListener;
use tower_http::validate_request::ValidateRequestHeaderLayer;
use tracing::info;

use crate::{
    config::Config,
    handlers::{QueryParams, RpcServer, State as AppState},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

async fn handle(
    Query(params): Query<QueryParams>,
    State(router): State<JrpcRouter<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    router.handle_request_with_params(body, params).await
}

pub fn server(cfg: Config) -> Router {
    let router = JrpcRouter::new(AppState::new(cfg).into_rpc());

    Router::new()
        .route("/", post(handle))
        .with_state(router)
        .layer(cors())
        .layer(ValidateRequestHeaderLayer::accept(mime::APPLICATION_JSON.as_ref()))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
