pub mod auth;

use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub host: String,
    pub port: u16,
    pub exp: u64,
    pub sub: String,
}
