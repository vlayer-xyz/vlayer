use std::{pin::Pin, sync::Arc};

use axum::{routing::post, Router};
use server_utils::{init_trace_layer, route, RequestIdLayer};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    config::Config,
    handlers::v_call::{v_call, Params},
};

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config)).await?;

    Ok(())
}

pub fn server(config: Config) -> Router {
    let config = Arc::new(config);
    let call_and_convert_to_json = |config: Arc<Config>, params: Params| async move {
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
