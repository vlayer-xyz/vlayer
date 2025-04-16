use axum::extract::FromRef;
use server_utils::jwt::axum::State as JwtState;

use super::AppState;

impl FromRef<AppState> for JwtState {
    #[allow(clippy::expect_used)]
    fn from_ref(AppState { config, .. }: &AppState) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        Self::new(config.public_key.clone(), config.algorithm)
    }
}
