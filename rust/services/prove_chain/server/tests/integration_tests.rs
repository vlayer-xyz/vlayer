use axum::http::StatusCode;
use prove_chain_server::{server::server, ServerConfig};
use serde_json::json;
use server_utils::{body_to_json, post};

#[tokio::test]
async fn http_not_found() {
    let app = server(ServerConfig::default());
    let empty_body = json!({});
    let response = post(app, "/non-existent", &empty_body).await;
    assert_eq!(StatusCode::NOT_FOUND, response.status());
}

#[tokio::test]
async fn method_not_found() {
    let app = server(ServerConfig::default());
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
    let app = server(ServerConfig::default());
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

mod v_prove_chain {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn success_dummy() {
        let app = server(ServerConfig::default());
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "v_proveChain",
            "params": {
                "chain_id": 1,
                "block_numbers": [1]
            }
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

    #[tokio::test]
    async fn no_block_numbers_error() {
        let app = server(ServerConfig::default());
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "v_proveChain",
            "params": {
                "chain_id": 1,
                "block_numbers": []
            }
        });
        let response = post(app, "/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32602,
                    "message": "Invalid params: empty list of block numbers provided - nothing to prove",
                    "data": null
                }
            }),
            body_to_json(response.into_body()).await
        );
    }

    #[tokio::test]
    async fn field_validation_error() {
        let app = server(ServerConfig::default());

        let valid_number = 42;
        let invalid_number = "";
        let req = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "v_proveChain",
            "params": {
                "chain_id": 1,
                "block_numbers": [valid_number, invalid_number]
            }
        });
        let response = post(app, "/", &req).await;

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            json!({
                "jsonrpc": "2.0",
                "id": 1,
                "error": {
                    "code": -32602,
                    "message": "invalid type: string \"\", expected u32",
                    "data": null
                }
            }),
            body_to_json(response.into_body()).await
        );
    }
}
