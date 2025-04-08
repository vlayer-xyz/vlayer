use axum::extract::FromRef;
use derive_new::new;
use server_utils::jwt::State as JwtState;
pub use server_utils::jwt::{Algorithm, DecodingKey};

use crate::server::State;

impl FromRef<State> for JwtState {
    fn from_ref(State { config, .. }: &State) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        Self::new(config.public_key.clone(), config.algorithm)
    }
}

#[derive(new, Clone)]
pub struct Config {
    public_key: DecodingKey,
    algorithm: Algorithm,
}
