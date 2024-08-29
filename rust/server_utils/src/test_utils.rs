use alloy_primitives::hex::ToHexExt;
use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, Response},
    Router,
};
use axum_jrpc::Value;
use ethers::types::Bytes;
use http_body_util::BodyExt;
use mime::APPLICATION_JSON;
use serde::Serialize;
use serde_json::to_string;
use tower::util::ServiceExt;

pub async fn body_to_string(body: Body) -> String {
    let body_bytes = body.collect().await.unwrap().to_bytes();
    String::from_utf8(body_bytes.to_vec()).unwrap()
}

pub async fn body_to_json(body: Body) -> Value {
    let body_bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body_bytes).unwrap()
}

pub fn function_selector(calldata: Bytes) -> String {
    let calldata_bytes = calldata.to_vec();
    let selector_bytes = &calldata_bytes.as_slice()[..4];
    selector_bytes.encode_hex_with_prefix()
}

pub async fn post<T: Serialize>(app: Router, url: &str, body: &T) -> Response<Body> {
    let request = Request::post(url)
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(to_string(body).unwrap()))
        .unwrap();
    app.oneshot(request).await.unwrap()
}
