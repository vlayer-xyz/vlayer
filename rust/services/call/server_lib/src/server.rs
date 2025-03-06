use std::iter::once;

use axum::{
    http::header::AUTHORIZATION,
    routing::{get, post},
    Router,
};
use derive_new::new;
use server_utils::{cors, init_trace_layer, RequestIdLayer, Router as JrpcRouter};
use tokio::net::TcpListener;
use tower_http::{
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::info;

#[cfg(feature = "jwt")]
use crate::jwt::handle;
#[cfg(not(feature = "jwt"))]
use crate::token::handle;
use crate::{
    config::Config,
    handlers::{RpcServer, State as AppState},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&config.socket_addr).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

#[derive(new, Clone)]
pub(super) struct State {
    pub config: Config,
    pub router: JrpcRouter<AppState>,
}

pub fn server(config: Config) -> Router {
    let router = State::new(config, JrpcRouter::new(AppState::default().into_rpc()));
    Router::new()
        .route("/", post(handle))
        .route_layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .route_layer(RequestIdLayer)
        .with_state(router)
        .route("/health", get(|| async { "OK" }))
        .layer(cors())
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(ValidateRequestHeaderLayer::accept(mime::APPLICATION_JSON.as_ref()))
}
