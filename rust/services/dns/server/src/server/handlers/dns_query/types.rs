use axum::{
    http::{header::CONTENT_TYPE, HeaderValue},
    response::IntoResponse,
};
use serde::Deserialize;

use super::MIME_DNS_JSON_CONTENT_TYPE;
use crate::verifiable_dns::record::Response;

#[derive(Deserialize, Debug)]
pub(super) struct Params {
    #[serde(rename(deserialize = "name"))]
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    _query_type: DNSQueryType,
}

#[derive(Deserialize, Debug)]
pub(super) enum DNSQueryType {
    #[allow(clippy::upper_case_acronyms)]
    #[serde(alias = "txt")]
    TXT,
}

impl IntoResponse for Response {
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
