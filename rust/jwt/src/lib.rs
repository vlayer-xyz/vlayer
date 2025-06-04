use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

pub use jsonwebtoken::{
    Algorithm as JwtAlgorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode,
    decode_header, encode, errors::Error as JwtError, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("empty string when parsing JWT claim")]
    EmptyString,
    #[error("JWT signing key not found: '{}'", .0.display())]
    JwtSigningKeyNotFound(PathBuf),
    #[error("JWT internal error: {0}")]
    Jwt(#[from] JwtError),
}

#[derive(
    Debug, Clone, Copy, Serialize, Deserialize, Default, Display, EnumString, VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
pub enum Algorithm {
    #[default]
    RS256,
    RS384,
    RS512,
    ES256,
    ES384,
    PS256,
    PS384,
    PS512,
    EdDSA,
}

impl From<Algorithm> for JwtAlgorithm {
    fn from(algorithm: Algorithm) -> Self {
        match algorithm {
            Algorithm::RS256 => Self::RS256,
            Algorithm::RS384 => Self::RS384,
            Algorithm::RS512 => Self::RS512,
            Algorithm::ES256 => Self::ES256,
            Algorithm::ES384 => Self::ES384,
            Algorithm::PS256 => Self::PS256,
            Algorithm::PS384 => Self::PS384,
            Algorithm::PS512 => Self::PS512,
            Algorithm::EdDSA => Self::EdDSA,
        }
    }
}

pub fn load_jwt_signing_key(
    public_key_path: impl AsRef<Path>,
    algorithm: Algorithm,
) -> Result<DecodingKey, Error> {
    let bytes = std::fs::read(public_key_path.as_ref())
        .map_err(|_| Error::JwtSigningKeyNotFound(public_key_path.as_ref().to_path_buf()))?;
    let key = match algorithm {
        Algorithm::RS256
        | Algorithm::RS384
        | Algorithm::RS512
        | Algorithm::PS256
        | Algorithm::PS384
        | Algorithm::PS512 => DecodingKey::from_rsa_pem(&bytes).map_err(Error::Jwt)?,
        Algorithm::ES256 | Algorithm::ES384 => {
            DecodingKey::from_ec_pem(&bytes).map_err(Error::Jwt)?
        }
        Algorithm::EdDSA => DecodingKey::from_ed_pem(&bytes).map_err(Error::Jwt)?,
    };
    Ok(key)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Claim {
    pub name: String,
    #[serde(default)]
    pub values: Vec<String>,
}

impl FromStr for Claim {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(Error::EmptyString);
        }
        let parts: Vec<&str> = s.split(':').collect();
        let name = parts[0].to_string();
        let values = parts[1..].iter().map(ToString::to_string).collect();
        Ok(Self { name, values })
    }
}
