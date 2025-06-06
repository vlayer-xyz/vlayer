use std::iter::once;

use axum::{
    Extension, Router,
    body::Bytes,
    extract::State as AxumState,
    http::header::AUTHORIZATION,
    response::IntoResponse,
    routing::{get, post},
};
use derive_new::new;
use server_utils::{
    RequestId, RequestIdLayer, Router as JrpcRouter, cors, init_trace_layer,
    jwt::axum::TokenExtractor,
};
use tokio::net::TcpListener;
use tower_http::{
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::info;

use crate::{
    config::Config,
    handlers::{Params, RpcServer, State as AppState},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&config.socket_addr).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

async fn handle(
    AxumState(State { router, config }): AxumState<State>,
    Extension(req_id): Extension<RequestId>,
    body: Bytes,
) -> impl IntoResponse {
    let params = Params::new(config, None, req_id);
    router.handle_request_with_params(body, params).await
}

async fn handle_with_auth(
    TokenExtractor(token): TokenExtractor,
    AxumState(State { router, config }): AxumState<State>,
    Extension(req_id): Extension<RequestId>,
    body: Bytes,
) -> impl IntoResponse {
    let params = Params::new(config, Some(token), req_id);
    router.handle_request_with_params(body, params).await
}

#[derive(new, Clone)]
pub(super) struct State {
    pub config: Config,
    pub router: JrpcRouter<AppState>,
}

pub fn server(config: Config) -> Router {
    let handler = if config.jwt_config.is_some() {
        post(handle_with_auth)
    } else {
        post(handle)
    };
    let router = State::new(config, JrpcRouter::new(AppState::default().into_rpc()));
    Router::new()
        .route("/", handler)
        .route_layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .route_layer(RequestIdLayer)
        .with_state(router)
        .route("/health", get(|| async { "OK" }))
        .layer(cors())
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(ValidateRequestHeaderLayer::accept(mime::APPLICATION_JSON.as_ref()))
}
