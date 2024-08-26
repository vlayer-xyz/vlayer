use alloy_primitives::hex::ToHexExt;
use axum::body::Body;
use axum_jrpc::Value;
use ethers::types::Bytes;
use http_body_util::BodyExt;

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
