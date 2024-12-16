use axum::{
    body::Bytes,
    extract::{Query, State},
    response::IntoResponse,
    routing::post,
    Router,
};
use server_utils::{init_trace_layer, RequestIdLayer, Router as JrpcRouter};
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, validate_request::ValidateRequestHeaderLayer};
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

    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    let cors = CorsLayer::permissive();
    Router::new()
        .route("/", post(handle))
        .with_state(router)
        .layer(ValidateRequestHeaderLayer::accept(mime::APPLICATION_JSON.as_ref()))
        .layer(cors)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
