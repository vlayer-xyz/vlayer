use axum::{
    extract::FromRef,
    http::StatusCode,
    response::{IntoResponse, Response},
};
pub use server_utils::jwt::{Algorithm, DecodingKey};
use server_utils::{
    ProofMode,
    jwt::{Environment, axum::State as JwtState},
};

use crate::{config::Config, server::State};

#[derive(Debug, thiserror::Error)]
#[error("Invalid environment in jwt: {0:?}")]
pub struct Error(Environment);

impl FromRef<State> for JwtState {
    #[allow(clippy::expect_used)]
    fn from_ref(State { config, .. }: &State) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        Self::new(config.public_key.clone(), config.algorithm)
    }
}

pub fn validate_environment(
    config: &Config,
    environment: Option<Environment>,
) -> Result<(), Error> {
    if let Some(environment) = environment {
        if environment
            != match config.proof_mode {
                ProofMode::Groth16 => Environment::Mainnet,
                ProofMode::Fake => Environment::Test,
            }
        {
            return Err(Error(environment));
        }
    }
    Ok(())
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}
