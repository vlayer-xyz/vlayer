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
#[error("Invalid environment in JWT: {0:?}, prover server proof mode: {1:?}")]
pub struct Error(Option<Environment>, ProofMode);

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
    environment
        .or(Some(Environment::Test))
        .map(proof_mode_from_environment)
        .filter(|mode| &config.proof_mode == mode)
        .ok_or(Error(environment, config.proof_mode))?;
    Ok(())
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

const fn proof_mode_from_environment(environment: Environment) -> ProofMode {
    match environment {
        Environment::Mainnet => ProofMode::Groth16,
        Environment::Test => ProofMode::Fake,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::tests::config_builder;

    #[test]
    fn validate_environment_mainnet() {
        let config = config_builder()
            .with_proof_mode(ProofMode::Groth16)
            .build()
            .unwrap();
        let environment = Some(Environment::Mainnet);
        let result = validate_environment(&config, environment);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_environment_test() {
        let config = config_builder()
            .with_proof_mode(ProofMode::Fake)
            .build()
            .unwrap();
        let environment = Some(Environment::Test);
        let result = validate_environment(&config, environment);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_environment_none() {
        let config = config_builder()
            .with_proof_mode(ProofMode::Fake)
            .build()
            .unwrap();
        let environment = None;
        let result = validate_environment(&config, environment);
        assert!(result.is_ok());
    }

    #[test]
    fn validate_environment_invalid() {
        let config = config_builder()
            .with_proof_mode(ProofMode::Groth16)
            .build()
            .unwrap();
        let environment = Some(Environment::Test);
        let result = validate_environment(&config, environment);
        assert!(result.is_err());
    }

    #[test]
    fn validate_environment_invalid_proof_mode() {
        let config = config_builder()
            .with_proof_mode(ProofMode::Fake)
            .build()
            .unwrap();
        let environment = Some(Environment::Mainnet);
        let result = validate_environment(&config, environment);
        assert!(result.is_err());
    }
}
