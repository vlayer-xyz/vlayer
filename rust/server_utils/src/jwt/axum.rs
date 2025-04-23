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
use derive_more::Deref;
use derive_new::new;
use jwt::{Algorithm, DecodingKey, Error as JwtError, Validation, decode, decode_header};
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Deref, Clone, Deserialize)]
pub struct ClaimsExtractor<T: Clone>(pub T);

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid JWT token")]
    InvalidToken,
    #[error("Missing JWT token")]
    MissingToken,
    #[error(transparent)]
    Jwt(JwtError),
}

#[derive(new, Clone)]
pub struct State {
    pub_key: DecodingKey,
    algorithm: Algorithm,
}

const HEADER_TYP: &str = "JWT";

impl<S, T> FromRequestParts<S> for ClaimsExtractor<T>
where
    for<'de> T: Clone + Deserialize<'de>,
    S: Send + Sync,
    State: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_ref(state);
        let Some(TypedHeader(Authorization(bearer))) = parts
            .extract::<Option<TypedHeader<Authorization<Bearer>>>>()
            .await
            .map_err(|_| Error::InvalidToken)?
        else {
            return Err(Error::MissingToken);
        };

        let header = decode_header(bearer.token()).map_err(|_| Error::InvalidToken)?;
        if header.typ.is_none_or(|typ| typ != HEADER_TYP) {
            return Err(Error::InvalidToken);
        }

        let mut validation = Validation::new(state.algorithm);
        validation.validate_exp = true;
        let token_data = decode::<ClaimsExtractor<T>>(bearer.token(), &state.pub_key, &validation)
            .map_err(Error::Jwt)?;
        Ok(token_data.claims)
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
