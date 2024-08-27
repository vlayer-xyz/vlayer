use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, Request, Response, StatusCode},
};
use block_proof::server;
use mime::APPLICATION_JSON;
use serde::Serialize;
use serde_json::{json, to_string};
use server_utils::body_to_json;
use tower::ServiceExt;

async fn post<T: Serialize>(url: &str, body: &T) -> Response<Body> {
    let app = server();
    let request = Request::post(url)
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(to_string(body).unwrap()))
        .unwrap();
    app.oneshot(request).await.unwrap()
}

#[tokio::test]
async fn http_not_found() {
    let empty_body = json!({});
    let response = post("/non_existing", &empty_body).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn method_not_found() {
    let req = json!({
        "method": "random_gibberish",
        "params": [],
        "id": 1,
        "jsonrpc": "2.0",
    });
    let response = post("/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method `random_gibberish` not found",
            }
        }),
        body_to_json(response.into_body()).await
    );
}

#[tokio::test]
async fn method_missing() {
    let req = json!({
        "params": [],
        "id": 1,
        "jsonrpc": "2.0",
    });
    let response = post("/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32600,
                "message": "Invalid request: Failed to deserialize the JSON body into the target type",
            }
        }),
        body_to_json(response.into_body()).await
    );
}
