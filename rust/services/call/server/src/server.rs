use std::{pin::Pin, sync::Arc};

use axum::{routing::post, Router};
use server_utils::{init_trace_layer, route, RequestIdLayer};
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    config::ServerConfig,
    handlers::v_call::{v_call, Params},
};

pub async fn serve(config: ServerConfig) -> anyhow::Result<()> {
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.host, config.port)).await?;

    info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

pub fn server(config: ServerConfig) -> Router {
    let config = Arc::new(config);
    let call_and_convert_to_json = |config: Arc<ServerConfig>, params: Params| async move {
        v_call(config.clone(), params).await.map(|x| x.to_json())
    };
    let v_call_handler =
        move |params| Box::pin(call_and_convert_to_json(config.clone(), params)) as Pin<Box<_>>;
    let jrpc_handler = |req| async move { route(req, "v_call", v_call_handler).await };

    //TODO: Lets decide do we need strict CORS policy or not and update this eventually
    let cors = CorsLayer::permissive();
    Router::new()
        .route("/", post(jrpc_handler))
        .layer(cors)
        .layer(init_trace_layer())
        // NOTE: RequestIdLayer should be added after the Trace layer
        .layer(RequestIdLayer)
}
