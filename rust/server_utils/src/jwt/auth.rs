use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use derive_more::Deref;
use derive_new::new;
use jsonwebtoken::{decode, errors::Error as JwtError, Validation};
pub use jsonwebtoken::{Algorithm, DecodingKey};
use serde::Deserialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid token")]
    InvalidToken,
    #[error(transparent)]
    Jwt(JwtError),
}

#[derive(new, Clone)]
pub struct State {
    pub_key: DecodingKey,
    algorithm: Algorithm,
}

#[derive(Deref, Clone, Deserialize)]
pub struct Claims<T: Clone>(pub T);

impl<S, T> FromRequestParts<S> for Claims<T>
where
    for<'de> T: Clone + Deserialize<'de>,
    S: Send + Sync,
    State: FromRef<S>,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = State::from_ref(state);
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::InvalidToken)?;
        let token_data =
            decode::<Claims<T>>(bearer.token(), &state.pub_key, &Validation::new(state.algorithm))
                .map_err(Error::Jwt)?;
        Ok(token_data.claims)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.to_string(),
        }));
        (StatusCode::BAD_REQUEST, body).into_response()
    }
}
