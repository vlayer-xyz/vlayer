#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use alloy_primitives::hex::ToHexExt;
use assert_json_diff::assert_json_include;
use axum::{
    Router,
    body::Body,
    http::{
        HeaderName, Request, Response,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
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

pub fn function_selector(calldata: &Bytes) -> String {
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

pub async fn post_with_bearer_auth<T: Serialize>(
    app: Router,
    url: &str,
    body: &T,
    token: &str,
) -> Response<Body> {
    let request = Request::post(url)
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::from(to_string(body).unwrap()))
        .unwrap();
    app.oneshot(request).await.unwrap()
}

pub async fn get(
    app: Router,
    url: &str,
    headers: &[(&HeaderName, &str)],
    query: &[(&str, &str)],
) -> Response<Body> {
    let query = query
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let request = Request::get(format!("{url}?{query}"));
    let request = headers
        .iter()
        .fold(request, |request, &(name, value)| request.header(name, value));

    let request = request.body("".to_string()).unwrap();

    app.oneshot(request).await.unwrap()
}

pub async fn assert_jrpc_ok(response: axum::response::Response, expected: impl Serialize) -> Value {
    let response_json = body_to_json(response.into_body()).await;
    if let Some(error) = response_json.get("error") {
        panic!("expected .result but found .error: {error}");
    }
    let result = response_json
        .get("result")
        .expect(".result not found in response body");
    assert_json_include!(expected: expected, actual: result);
    response_json
}

pub async fn assert_jrpc_err(response: axum::response::Response, code: i32, msg: &str) -> Value {
    let response_json = body_to_json(response.into_body()).await;
    if let Some(result) = response_json.get("result") {
        panic!("expected .error but found .result: {result}");
    }
    let error = response_json
        .get("error")
        .expect(".error not found in response body");
    assert_json_include!(expected: serde_json::json!({"code": code, "message": msg}), actual: error);
    response_json
}
