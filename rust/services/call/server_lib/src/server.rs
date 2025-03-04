use std::iter::once;

use axum::{
    body::Bytes,
    extract::State as AxumState,
    http::header::AUTHORIZATION,
    response::IntoResponse,
    routing::{get, post},
    Extension, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use derive_new::new;
use server_utils::{cors, init_trace_layer, RequestId, RequestIdLayer, Router as JrpcRouter};
use tokio::net::TcpListener;
use tower_http::{
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::info;

use crate::{
    config::{AuthMode, Config},
    handlers::{Params, RpcServer, State as AppState},
    user_token::Token as UserToken,
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

#[derive(new, Clone)]
pub(super) struct State {
    pub config: Config,
    pub router: JrpcRouter<AppState>,
}

async fn handle(
    user_token: Option<TypedHeader<Authorization<Bearer>>>,
    AxumState(State { router, .. }): AxumState<State>,
    Extension(req_id): Extension<RequestId>,
    body: Bytes,
) -> impl IntoResponse {
    let user_token: Option<UserToken> = user_token.map(|TypedHeader(user_token)| user_token.into());
    let params = Params::new(user_token, req_id);
    router.handle_request_with_params(body, params).await
}

pub fn server(config: Config) -> Router {
    let handler = match config.auth_mode() {
        #[cfg(feature = "jwt")]
        AuthMode::Jwt => post(crate::jwt::handle),
        AuthMode::Token => post(handle),
    };
    let router = State::new(config.clone(), JrpcRouter::new(AppState::new(config).into_rpc()));

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
