mod handlers;

use axum::Router;
use server_utils::{init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tracing::info;

use crate::config::Config;

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server()).await?;

    Ok(())
}
pub fn server() -> axum::Router {
    handlers::handlers()
        .into_iter()
        .fold(Router::new(), |router, (path, handler)| router.route(path, handler))
        .layer(init_trace_layer())
        .layer(RequestIdLayer)
}
