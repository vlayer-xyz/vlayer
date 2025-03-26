use axum::{RequestPartsExt, extract::OptionalFromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use derive_more::{Deref, From};
use serde::Deserialize;

#[derive(From, Clone, Debug, Deref, Deserialize)]
pub struct Token(String);

impl From<Authorization<Bearer>> for Token {
    fn from(value: Authorization<Bearer>) -> Self {
        Self(value.token().into())
    }
}

#[derive(Deref, Clone, Deserialize)]
pub(super) struct TokenExtractor(pub Token);

impl<S> OptionalFromRequestParts<S> for TokenExtractor
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .ok()
            .map(|bearer| TokenExtractor(bearer.token().to_string().into())))
    }
}
