use std::path::{Path, PathBuf};

use derive_builder::Builder;
pub use jsonwebtoken::{
    Algorithm as JwtAlgorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode,
    decode_header, encode, errors::Error as JwtError, get_current_timestamp,
};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString, VariantNames};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, EnumString, Display)]
#[serde(rename_all = "lowercase")]
#[strum(ascii_case_insensitive)]
#[strum(serialize_all = "lowercase")]
pub enum Environment {
    #[default]
    Test,
    Production,
}

#[derive(Builder, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[builder(pattern = "owned")]
pub struct Claims {
    #[builder(setter(into, strip_option), default)]
    pub host: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub port: Option<u16>,
    pub exp: u64,
    pub sub: String,
    #[builder(setter(into), default)]
    pub environment: Option<Environment>,
}

#[allow(clippy::unwrap_used)]
pub mod test_helpers {
    use super::*;

    pub struct TokenArgs<'a> {
        pub secret: &'a str,
        pub invalid_after: i64,
        pub subject: &'a str,
        pub host: Option<&'a str>,
        pub port: Option<u16>,
        pub environment: Option<Environment>,
    }

    #[allow(clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    pub fn token(args: &TokenArgs) -> String {
        let key = EncodingKey::from_secret(args.secret.as_bytes());
        let ts = get_current_timestamp() as i64 + args.invalid_after;
        let mut claims_builder = ClaimsBuilder::default()
            .exp(ts as u64)
            .sub(args.subject.to_string())
            .environment(args.environment);
        if let Some(host) = args.host {
            claims_builder = claims_builder
                .host(host.to_string())
                .port(args.port.unwrap_or(DEFAULT_WEB_PROOF_PORT))
        }

        let claims = claims_builder.build().unwrap();

        encode(&Header::default(), &claims, &key).unwrap()
    }

    pub const JWT_SECRET: &str = "deadbeef";
    pub const DEFAULT_WEB_PROOF_PORT: u16 = 443;
}

#[cfg(test)]
mod tests {
    use test_helpers::{JWT_SECRET, TokenArgs, token};

    use super::*;

    fn decoding_key() -> DecodingKey {
        DecodingKey::from_secret(JWT_SECRET.as_bytes())
    }

    #[test]
    fn decodes_without_web_proof_claims() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: None,
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: None,
        });
        let claims: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert!(claims.claims.host.is_none());
        assert!(claims.claims.port.is_none());
    }

    #[test]
    fn decodes_with_web_proof_claims() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: Some("xyz.xyz"),
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: None,
        });
        let claims: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert_eq!(claims.claims.host, Some("xyz.xyz".to_string()));
        assert_eq!(claims.claims.port, Some(443));
    }

    #[test]
    fn decodes_as_test() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: None,
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: Some(Environment::Test),
        });
        let token_data: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert_eq!(token_data.claims.environment, Some(Environment::Test));
    }

    #[test]
    fn decodes_as_production() {
        let jwt = token(&TokenArgs {
            secret: JWT_SECRET,
            host: None,
            port: None,
            invalid_after: 0,
            subject: "1234",
            environment: Some(Environment::Production),
        });
        let token_data: TokenData<Claims> =
            decode(&jwt, &decoding_key(), &Validation::default()).unwrap();
        assert_eq!(token_data.claims.environment, Some(Environment::Production));
    }
}
