use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Config {
    socket_addr: SocketAddr,
}

impl Config {
    pub const fn new(socket_addr: SocketAddr) -> Self {
        Config { socket_addr }
    }

    pub const fn socket_addr(&self) -> SocketAddr {
        self.socket_addr
    }
}
