mod handlers;
mod jwt;

use axum::Router;
use server_utils::{RequestIdLayer, cors, init_trace_layer};
use tokio::net::TcpListener;
use tracing::{info, warn};
use verifiable_dns::{ExternalProvider, VerifiableDNSResolver};

use crate::config::Config;

#[derive(Clone)]
struct AppState {
    vdns_resolver: VerifiableDNSResolver,
    config: Config,
}

impl AppState {
    fn new(config: Config) -> Self {
        let providers = [ExternalProvider::google_provider(), ExternalProvider::dns_sb_provider()];
        let vdns_resolver = match config.private_key.as_deref() {
            Some(key) => VerifiableDNSResolver::new(providers).with_key(key),
            None => {
                warn!("Private key not provided, using default resolver key");
                VerifiableDNSResolver::new(providers)
            }
        };

        Self {
            vdns_resolver,
            config,
        }
    }
}

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let listener = TcpListener::bind(&config.socket_addr).await?;

    let state = AppState::new(config.clone());

    info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, server(config, state)).await?;

    Ok(())
}

fn server(config: Config, state: AppState) -> Router {
    handlers::handlers(config)
        .into_iter()
        .fold(Router::new(), |router, (path, handler)| router.route(path, handler))
        .with_state(state)
        .layer(cors())
        .layer(init_trace_layer())
        .layer(RequestIdLayer)
}

#[cfg(test)]
mod test_helpers {
    use server_utils::jwt::test_helpers::default_config as default_jwt_config;

    use super::*;
    use crate::config::ConfigBuilder;

    pub fn app() -> Router {
        let config = ConfigBuilder::default().build();
        let state = AppState::new(config.clone());
        server(config, state)
    }

    pub fn app_with_jwt_auth() -> Router {
        let config = ConfigBuilder::default()
            .with_jwt_config(default_jwt_config())
            .build();
        let state = AppState::new(config.clone());
        server(config, state)
    }
}
