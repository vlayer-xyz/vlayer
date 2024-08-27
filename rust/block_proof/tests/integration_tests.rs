use axum::{body::Body, http::StatusCode, serve};
use block_proof::server;
use reqwest::{header, Client, Method, Response};
use serde::Serialize;
use serde_json::{from_slice, json, to_string, Value};
use server_utils::body_to_json;

async fn post(url: &str, body: Value) -> Response {
    let app = server();
    let addr = "127.0.0.1:4000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    serve(listener, app).await.unwrap();

    Client::new()
        .post(addr.to_string() + url)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .unwrap() // TODO: handle error?
}

#[tokio::test]
async fn http_not_found() {
    let empty_body = json!({});
    let response = post("/non_existing", empty_body).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn method_missing() {
    let req = json!({
        "params": [],
        "id": 1,
        "jsonrpc": "2.0",
    });

    let response = post("/", req).await;
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method missing",
                "data": null
            }
        }),
        response.json::<Value>().await.unwrap()
    );
}

#[tokio::test]
async fn method_not_found() {
    let req = json!({
        "method": "random_gibberish",
        "params": [],
        "id": 1,
        "jsonrpc": "2.0",
    });
    let response = post("/", req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method \"random_gibberish\" not found",
                "data": null
            }
        }),
        response.json::<Value>().await.unwrap()
    );
}
