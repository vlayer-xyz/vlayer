use serde::{Deserialize, Serialize};
pub use server_utils::jwt::auth::{Algorithm, DecodingKey};

use axum::{
    body::Bytes,
    extract::{FromRef, State as AxumState},
    response::IntoResponse,
    Extension,
};
use derive_new::new;
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
        Self::new(config.public_key(), config.algorithm())
    }
}

pub(super) async fn handle(
    _: TokenClaims<Claims>,
    AxumState(State { router, .. }): AxumState<State>,
    Extension(req_id): Extension<RequestId>,
    body: Bytes,
) -> impl IntoResponse {
    let params = Params::new(None, req_id);
    router.handle_request_with_params(body, params).await
}

pub trait ConfigExt {
    fn public_key(&self) -> DecodingKey;
    fn algorithm(&self) -> Algorithm;
}

impl ConfigExt for crate::config::Config {
    fn public_key(&self) -> DecodingKey {
        self.jwt_config().map_or(
            DecodingKey::from_secret(b"0xdeadbeef"),
            |Config { public_key, .. }| {
                DecodingKey::from_rsa_pem(public_key.as_bytes()).expect("public key in PEM format")
            },
        )
    }

    fn algorithm(&self) -> Algorithm {
        self.jwt_config()
            .map_or(Algorithm::default(), |Config { algorithm, .. }| algorithm)
    }
}

#[derive(new, Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    public_key: String,
    algorithm: Algorithm,
}
