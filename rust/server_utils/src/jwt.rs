mod axum;

pub use axum::{Algorithm, DecodingKey, Error, State};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub host: String,
    pub port: u16,
    pub exp: u64,
    pub sub: String,
}
