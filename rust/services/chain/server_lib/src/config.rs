use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub listen_addr: SocketAddr,
}

impl ServerConfig {
    pub const fn new(listen_addr: SocketAddr) -> Self {
        Self { listen_addr }
    }
}

impl Default for ServerConfig {
    #[allow(clippy::unwrap_used)]
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0:3001".parse().unwrap(),
        }
    }
}
