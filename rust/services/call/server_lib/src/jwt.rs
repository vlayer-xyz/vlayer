use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts, OptionalFromRequestParts},
    http::request::Parts,
};
use derive_more::Deref;
use derive_new::new;
use serde::Deserialize;
pub use server_utils::jwt::{Algorithm, DecodingKey};
use server_utils::jwt::{Claims, ClaimsExtractor, Error as JwtError, State as JwtState};

use crate::{
    server::State,
    token::{Token, TokenExtractor as RawTokenExtractor},
};

impl FromRef<State> for JwtState {
    fn from_ref(State { config, .. }: &State) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        Self::new(config.public_key.clone(), config.algorithm)
    }
}

#[derive(Deref, Clone, Deserialize)]
pub(super) struct TokenExtractor(pub Token);

impl<S> OptionalFromRequestParts<S> for TokenExtractor
where
    S: Send + Sync,
    JwtState: FromRef<S>,
{
    type Rejection = JwtError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        match ClaimsExtractor::from_request_parts(parts, state).await {
            Ok(ClaimsExtractor(Claims { sub, .. })) => Ok(Some(TokenExtractor(sub.into()))),
            Err(JwtError::InvalidToken) => Ok(parts
                .extract::<Option<RawTokenExtractor>>()
                .await
                .ok()
                .flatten()
                .map(|RawTokenExtractor(token)| TokenExtractor(token))),
            Err(e) => Err(e),
        }
    }
}

#[derive(new, Clone)]
pub struct Config {
    public_key: DecodingKey,
    algorithm: Algorithm,
}
