use axum::body::Body;
use axum_jrpc::Value;
use ethers::types::U256;
use http_body_util::BodyExt;

pub async fn body_to_string(body: Body) -> anyhow::Result<String> {
    let body_bytes = body.collect().await?.to_bytes();
    Ok(String::from_utf8(body_bytes.to_vec())?)
}

pub async fn body_to_json(body: Body) -> Value {
    let body_bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body_bytes).unwrap()
}

pub fn bool_to_vec32(value: bool) -> Vec<u8> {
    let mut bytes = [0u8; 32];
    bytes[31] = value as u8;
    bytes.to_vec()
}

pub fn u256_to_vec32(value: U256) -> Vec<u8> {
    let mut bytes = [0u8; 32];
    value.to_big_endian(&mut bytes);
    bytes.to_vec()
}
