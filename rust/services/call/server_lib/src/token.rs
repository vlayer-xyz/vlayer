use axum::{
    body::Bytes,
    extract::{OptionalFromRequestParts, State as AxumState},
    http::request::Parts,
    response::IntoResponse,
    Extension, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use derive_more::{Deref, From};
use serde::Deserialize;
use server_utils::RequestId;

use crate::{handlers::Params, server::State};

#[derive(From, Clone, Debug, Deref, Deserialize)]
pub struct Token(String);

impl From<Authorization<Bearer>> for Token {
    fn from(value: Authorization<Bearer>) -> Self {
        Self(value.token().into())
    }
}

pub(super) async fn handle(
    token: Option<TokenExtractor>,
    AxumState(State { router, config }): AxumState<State>,
    Extension(req_id): Extension<RequestId>,
    body: Bytes,
) -> impl IntoResponse {
    let params = Params::new(config, token.as_deref().cloned(), req_id);
    router.handle_request_with_params(body, params).await
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
