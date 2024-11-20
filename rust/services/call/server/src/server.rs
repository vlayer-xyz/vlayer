use std::{future::Future, pin::Pin, sync::Arc};

use axum::{extract::State, response::IntoResponse, routing::post, Router};
use axum_jrpc::JsonRpcExtractor;
use server_utils::{init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{config::Config, handlers::v_call::v_call};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

async fn handle_jrpc(
    State(router): State<server_utils::Router>,
    request: JsonRpcExtractor,
) -> impl IntoResponse {
    router.handle_request(request).await
}

pub fn server(config: Config) -> Router {
    let config = Arc::new(config);
    let mut jrpc_router = server_utils::Router::default();
    jrpc_router.add_handler("v_call", move |params| -> Pin<Box<dyn Future<Output = _> + Send>> {
        let config = config.clone();
        Box::pin(async move { v_call(config, params).await.map(|x| x.to_json()) })
    });

    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    let cors = CorsLayer::permissive();
    Router::new()
        .route("/", post(handle_jrpc))
        .with_state(jrpc_router)
        .layer(cors)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
