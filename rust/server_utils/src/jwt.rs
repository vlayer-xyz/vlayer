pub mod auth;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub host: String,
    pub port: u16,
    pub exp: u64,
}
