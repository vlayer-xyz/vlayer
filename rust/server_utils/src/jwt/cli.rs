use std::path::PathBuf;

use clap::Parser;
use derive_new::new;
use jwt::{Algorithm, DecodingKey, Error as JwtError};
use thiserror::Error;
use tracing::{info, warn};

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(long, group = "auth")]
    pub jwt_public_key: Option<PathBuf>,

    #[arg(long, requires = "auth", default_value = "RS256")]
    pub jwt_algorithm: Option<Algorithm>,
}

#[derive(new, Clone)]
pub struct Config {
    pub public_key: DecodingKey,
    pub algorithm: Algorithm,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Public key '{}' not found", .0.display())]
    InvalidPublicKey(PathBuf),

    #[error(transparent)]
    Jwt(JwtError),
}

impl TryFrom<Args> for Option<Config> {
    type Error = Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        let Args {
            jwt_public_key,
            jwt_algorithm,
        } = args;
        let Some(jwt_public_key) = jwt_public_key else {
            warn!("Running without authorization.");
            return Ok(None);
        };
        let public_key = std::fs::read_to_string(&jwt_public_key)
            .map_err(|_| Error::InvalidPublicKey(jwt_public_key.clone()))?;
        let public_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).map_err(Error::Jwt)?;
        let algorithm = jwt_algorithm.unwrap_or_default();
        info!(
            "Using JWT-based authorization with public key '{}' and algorithm '{algorithm:#?}'.",
            jwt_public_key.display()
        );
        Ok(Some(Config::new(public_key, algorithm)))
    }
}
