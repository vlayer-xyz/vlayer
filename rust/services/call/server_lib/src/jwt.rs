use axum::extract::FromRef;
use server_utils::jwt::config::Config as JwtConfig;
pub use server_utils::jwt::{Algorithm, DecodingKey};

use crate::server::State;

impl FromRef<State> for JwtConfig {
    #[allow(clippy::expect_used)]
    fn from_ref(State { config, .. }: &State) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        config.clone()
    }
}
