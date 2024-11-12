use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub socket_addr: SocketAddr,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            socket_addr: "0.0.0.0:3000".parse().unwrap(),
        }
    }
}
