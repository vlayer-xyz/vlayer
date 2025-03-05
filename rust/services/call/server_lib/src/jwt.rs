use axum::{
    body::Bytes,
    extract::{FromRef, State as AxumState},
    response::IntoResponse,
    Extension,
};
use derive_new::new;
pub use server_utils::jwt::auth::{Algorithm, DecodingKey};
use server_utils::{
    jwt::{
        auth::{Claims as TokenClaims, State as JwtState},
        Claims,
    },
    RequestId,
};

use crate::{handlers::Params, server::State};

impl FromRef<State> for JwtState {
    fn from_ref(State { config, .. }: &State) -> Self {
        let config = config
            .jwt_config
            .as_ref()
            .expect("public key and algorithm must be specified at the config level");
        Self::new(config.public_key.clone(), config.algorithm)
    }
}

pub(super) async fn handle(
    TokenClaims(Claims { sub, .. }): TokenClaims<Claims>,
    AxumState(State { router, config }): AxumState<State>,
    Extension(req_id): Extension<RequestId>,
    body: Bytes,
) -> impl IntoResponse {
    let params = Params::new(config, Some(sub.into()), req_id);
    router.handle_request_with_params(body, params).await
}

#[derive(new, Clone)]
pub struct Config {
    public_key: DecodingKey,
    algorithm: Algorithm,
}
