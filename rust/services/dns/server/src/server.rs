mod handlers;

use axum::Router;
use server_utils::{RequestIdLayer, cors, init_trace_layer};
use tokio::net::TcpListener;
use tracing::{info, warn};
use verifiable_dns::{ExternalProvider, VerifiableDNSResolver};

use crate::config::Config;

#[derive(Clone)]
struct AppState {
    vdns_resolver: VerifiableDNSResolver,
}

impl AppState {
    fn new(private_key: Option<&str>) -> Self {
        let providers = [ExternalProvider::google_provider(), ExternalProvider::dns_sb_provider()];
        let vdns_resolver = match private_key {
            Some(key) => VerifiableDNSResolver::new(providers).with_key(key),
            None => {
                warn!("Private key not provided, using default resolver key");
                VerifiableDNSResolver::new(providers)
            }
        };

        Self { vdns_resolver }
    }
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(config.socket_addr()).await?;

    let state = AppState::new(config.private_key());

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
        let state = AppState::new(None);
        server(state)
    }
}
