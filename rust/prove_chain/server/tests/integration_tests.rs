use axum::http::StatusCode;
use prove_chain_server::server;
use serde_json::json;
use server_utils::{body_to_json, post};

#[tokio::test]
async fn http_not_found() {
    let app = server();
    let empty_body = json!({});
    let response = post(app, "/non-existent", &empty_body).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn method_not_found() {
    let app = server();
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "random_gibberish",
        "params": []
    });
    let response = post(app, "/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "error": {
                "code": -32601,
                "message": "Method `random_gibberish` not found",
                "data": null
            }
        }),
        body_to_json(response.into_body()).await
    );
}

#[tokio::test]
async fn method_missing() {
    let app = server();
    let req = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "params": []
    });
    let response = post(app, "/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": null,
            "error": {
                "code": -32600,
                "message": "missing field `method` at line 1 column 36",
                "data": null
            }
        }),
        body_to_json(response.into_body()).await
    );
}

#[tokio::test]
async fn success_dummy() {
    let app = server();
    let req = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "v_proveChain",
        "params": []
    });
    let response = post(app, "/", &req).await;

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        json!({
            "jsonrpc": "2.0",
            "id": 1,
            "result": null
        }),
        body_to_json(response.into_body()).await
    );
}
