use std::net::{IpAddr, Ipv4Addr, SocketAddr as RawSocketAddr};

use server_utils::jwt::cli::Config as JwtConfig;

#[derive(Clone)]
#[allow(clippy::struct_field_names)]
pub struct Config {
    pub socket_addr: RawSocketAddr,
    pub private_key: Option<String>,
    pub jwt_config: Option<JwtConfig>,
}

pub struct SocketAddr(RawSocketAddr);

impl Default for SocketAddr {
    fn default() -> Self {
        Self(RawSocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3002))
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    socket_addr: SocketAddr,
    private_key: Option<String>,
    jwt_config: Option<JwtConfig>,
}

impl ConfigBuilder {
    #[must_use]
    pub const fn with_socket_addr(mut self, addr: RawSocketAddr) -> Self {
        self.socket_addr.0 = addr;
        self
    }

    #[must_use]
    pub fn with_private_key(mut self, key: impl Into<Option<String>>) -> Self {
        self.private_key = key.into();
        self
    }

    #[must_use]
    pub fn with_jwt_config(mut self, jwt_config: impl Into<Option<JwtConfig>>) -> Self {
        self.jwt_config = jwt_config.into();
        self
    }

    pub fn build(self) -> Config {
        let socket_addr = self.socket_addr.0;
        let private_key = self.private_key;
        let jwt_config = self.jwt_config;
        Config {
            socket_addr,
            private_key,
            jwt_config,
        }
    }
}
