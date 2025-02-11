use axum::{body::Body, http::Response, Router};
use call_server_lib::{server, Config};
use serde::Serialize;
use server_utils::{post, post_with_bearer_auth};

pub(crate) struct Server(Router);

impl Server {
    pub(crate) fn new(config: Config) -> Self {
        Self(server(config))
    }

    pub(crate) async fn post(&self, url: &str, body: impl Serialize) -> Response<Body> {
        post(self.0.clone(), url, &body).await
    }

    pub(crate) async fn post_with_bearer_auth(
        &self,
        url: &str,
        body: impl Serialize,
        token: &str,
    ) -> Response<Body> {
        post_with_bearer_auth(self.0.clone(), url, &body, token).await
    }
}
