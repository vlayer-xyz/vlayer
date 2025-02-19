use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    socket_addr: SocketAddr,
    private_key: Option<String>,
}

impl Config {
    pub const fn new(socket_addr: SocketAddr, private_key: Option<String>) -> Self {
        Config {
            socket_addr,
            private_key,
        }
    }

    pub const fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }

    pub fn private_key(&self) -> Option<&str> {
        self.private_key.as_deref()
    }
}
