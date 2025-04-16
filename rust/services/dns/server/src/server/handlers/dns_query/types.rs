use axum::{
    http::{HeaderValue, header::CONTENT_TYPE},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use verifiable_dns::{MIME_DNS_JSON_CONTENT_TYPE, Query, Response};

#[derive(Deserialize, Debug)]
pub(super) struct Params {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    _query_type: DNSQueryType,
}

impl From<Params> for Query {
    fn from(val: Params) -> Self {
        val.name.into()
    }
}

#[derive(Deserialize, Debug)]
pub(super) enum DNSQueryType {
    #[allow(clippy::upper_case_acronyms)]
    #[serde(alias = "txt")]
    TXT,
}

#[derive(Serialize, Debug)]
pub(super) struct ServerResponse(pub(super) Response);

impl IntoResponse for ServerResponse {
    #[allow(clippy::expect_used)]
    fn into_response(self) -> axum::response::Response {
        let mut response = serde_json::to_string(&self)
            .expect("Failed to serialize DNS response")
            .into_response();

        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static(MIME_DNS_JSON_CONTENT_TYPE));

        response
    }
}
