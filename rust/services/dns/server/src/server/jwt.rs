use axum::extract::FromRef;
use server_utils::jwt::config::Config as JwtConfig;

use super::AppState;

impl FromRef<AppState> for JwtConfig {
    #[allow(clippy::expect_used)]
    fn from_ref(AppState { config, .. }: &AppState) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        config.clone()
    }
}
