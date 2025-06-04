use axum::{
    Json, RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use derive_more::{Deref, From};
use derive_new::new;
use jwt::{JwtError, Validation, decode, decode_header};
use serde::Deserialize;
use serde_json::{Value, json};
use thiserror::Error;
use tracing::error;

use super::config::{Config, ValidationError};

#[derive(new, From, Clone, Debug, Deref, Deserialize)]
pub struct Token(String);

#[derive(Deref, Clone, Deserialize)]
pub struct TokenExtractor(pub Token);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid JWT token")]
    InvalidToken,
    #[error("Missing JWT token")]
    MissingToken,
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error(transparent)]
    Jwt(#[from] JwtError),
}

const HEADER_TYP: &str = "JWT";

impl<S> FromRequestParts<S> for TokenExtractor
where
    S: Send + Sync,
    Config: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = Config::from_ref(state);
        let Some(TypedHeader(Authorization(bearer))) = parts
            .extract::<Option<TypedHeader<Authorization<Bearer>>>>()
            .await
            .map_err(|_| Error::InvalidToken)?
        else {
            return Err(Error::MissingToken);
        };

        let token = bearer.token();
        let header = decode_header(token).map_err(|_| Error::InvalidToken)?;
        if header.typ.is_none_or(|typ| typ != HEADER_TYP) {
            return Err(Error::InvalidToken);
        }

        let mut validation = Validation::new(config.algorithm);
        validation.validate_exp = true;
        let token_data =
            decode::<Value>(token, &config.public_key, &validation).map_err(Error::Jwt)?;
        config.validate(&token_data.claims)?;
        Ok(Self(token.to_string().into()))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = json!({
            "error": self.to_string(),
        });
        error!("authorization error: {body}");
        (StatusCode::UNAUTHORIZED, Json(body)).into_response()
    }
}
