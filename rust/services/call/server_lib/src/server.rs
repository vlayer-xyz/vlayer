use std::iter::once;

use axum::{
    body::Bytes, extract::State, http::header::AUTHORIZATION, response::IntoResponse,
    routing::post, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use server_utils::{cors, init_trace_layer, RequestIdLayer, Router as JrpcRouter};
use tokio::net::TcpListener;
use tower_http::{
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::info;

use crate::{
    config::Config,
    handlers::{RpcServer, State as AppState},
    user_token::Token as UserToken,
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

async fn handle(
    user_token: Option<TypedHeader<Authorization<Bearer>>>,
    State(router): State<JrpcRouter<AppState>>,
    body: Bytes,
) -> impl IntoResponse {
    let user_token: Option<UserToken> = user_token.map(|TypedHeader(user_token)| user_token.into());
    router.handle_request_with_params(body, user_token).await
}

pub fn server(cfg: Config) -> Router {
    let router = JrpcRouter::new(AppState::new(cfg).into_rpc());

    Router::new()
        .route("/", post(handle))
        .with_state(router)
        .layer(cors())
        .layer(SetSensitiveRequestHeadersLayer::new(once(AUTHORIZATION)))
        .layer(ValidateRequestHeaderLayer::accept(mime::APPLICATION_JSON.as_ref()))
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
