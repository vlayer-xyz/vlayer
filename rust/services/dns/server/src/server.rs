mod handlers;

use axum::Router;
use server_utils::{cors, init_trace_layer, RequestIdLayer};
use tokio::net::TcpListener;
use tracing::info;

use crate::config::Config;

#[derive(Clone)]
struct AppState;

impl AppState {
    fn new() -> Self {
        Self
    }
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    let state = AppState::new();

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(state)).await?;

    Ok(())
}

fn server(state: AppState) -> Router {
    handlers::handlers()
        .into_iter()
        .fold(Router::new(), |router, (path, handler)| router.route(path, handler))
        .with_state(state)
        .layer(cors())
        .layer(init_trace_layer())
        .layer(RequestIdLayer)
}

#[cfg(test)]
mod test_helpers {

    use super::*;

    pub fn app() -> Router {
        let state = AppState::new();
        server(state)
    }
}
